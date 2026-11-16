use {
    anchor_lang::{
        error::ErrorCode, prelude::Pubkey, solana_program::instruction::Instruction,
        system_program, AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::{types::TransactionResult, LiteSVM},
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    todo::{accounts, error::TodoError, instruction, state::Task, TASK_SEED},
};

const ONE_SOL: u64 = 1_000_000_000;
const FEE: u64 = 5_000;

// A fresh runtime with our program loaded, and a user with 1 SOL.
fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    let program = include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/todo.so"));
    svm.add_program(todo::id(), program).unwrap();
    let user = Keypair::new();
    svm.airdrop(&user.pubkey(), ONE_SOL).unwrap();
    (svm, user)
}

// Same seeds as the program: "task" + owner + task ID.
fn task_pda(owner: &Pubkey, task_id: u64) -> Pubkey {
    let seeds = [TASK_SEED, owner.as_ref(), &task_id.to_le_bytes()];
    Pubkey::find_program_address(&seeds, &todo::id()).0
}

fn send(
    svm: &mut LiteSVM,
    signer: &Keypair,
    data: impl InstructionData,
    accounts: impl ToAccountMetas,
) -> TransactionResult {
    let ix = Instruction::new_with_bytes(todo::id(), &data.data(), accounts.to_account_metas(None));
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&signer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[signer]).unwrap();
    svm.send_transaction(tx)
}

fn create_task(svm: &mut LiteSVM, owner: &Keypair, task_id: u64, text: &str) -> TransactionResult {
    let data = instruction::CreateTask {
        task_id,
        description: text.to_string(),
    };
    let task = task_pda(&owner.pubkey(), task_id);
    let system_program = system_program::ID;
    send(
        svm,
        owner,
        data,
        accounts::CreateTask {
            owner: owner.pubkey(),
            task,
            system_program,
        },
    )
}

fn update_task(svm: &mut LiteSVM, owner: &Keypair, task: Pubkey, text: &str) -> TransactionResult {
    let data = instruction::UpdateTask {
        description: text.to_string(),
    };
    send(
        svm,
        owner,
        data,
        accounts::UpdateTask {
            owner: owner.pubkey(),
            task,
        },
    )
}

fn complete_task(svm: &mut LiteSVM, owner: &Keypair, task: Pubkey) -> TransactionResult {
    let data = instruction::CompleteTask {};
    send(
        svm,
        owner,
        data,
        accounts::UpdateTask {
            owner: owner.pubkey(),
            task,
        },
    )
}

fn delete_task(svm: &mut LiteSVM, owner: &Keypair, task: Pubkey) -> TransactionResult {
    let data = instruction::DeleteTask {};
    send(
        svm,
        owner,
        data,
        accounts::DeleteTask {
            owner: owner.pubkey(),
            task,
        },
    )
}

fn read_task(svm: &LiteSVM, task: &Pubkey) -> Task {
    let account = svm.get_account(task).unwrap();
    Task::try_deserialize(&mut account.data.as_slice()).unwrap()
}

fn assert_error(result: TransactionResult, error: impl Into<u32>) {
    let logs = result.unwrap_err().meta.logs.join("\n");
    let number = format!("Error Number: {}.", error.into());
    assert!(logs.contains(&number), "expected {number}\n{logs}");
}

#[test]
fn owner_can_create_edit_complete_and_delete() {
    let (mut svm, user) = setup();
    let task = task_pda(&user.pubkey(), 1);

    create_task(&mut svm, &user, 1, "Write the script").unwrap();
    assert_eq!(read_task(&svm, &task).description, "Write the script");

    update_task(&mut svm, &user, task, "Record the video").unwrap();
    assert_eq!(read_task(&svm, &task).description, "Record the video");

    complete_task(&mut svm, &user, task).unwrap();
    assert!(read_task(&svm, &task).completed);

    let rent = svm.get_balance(&task).unwrap();
    let before = svm.get_balance(&user.pubkey()).unwrap();
    delete_task(&mut svm, &user, task).unwrap();

    assert_eq!(svm.get_balance(&task).unwrap_or(0), 0);
    assert_eq!(
        svm.get_balance(&user.pubkey()).unwrap(),
        before + rent - FEE
    );
}

#[test]
fn another_wallet_cannot_edit_my_task() {
    let (mut svm, user) = setup();
    let attacker = Keypair::new();
    svm.airdrop(&attacker.pubkey(), ONE_SOL).unwrap();
    let task = task_pda(&user.pubkey(), 1);
    create_task(&mut svm, &user, 1, "My task").unwrap();

    let result = update_task(&mut svm, &attacker, task, "Hacked");

    assert_error(result, ErrorCode::ConstraintSeeds);
    assert_eq!(read_task(&svm, &task).description, "My task");
}

#[test]
fn description_over_280_bytes_fails() {
    let (mut svm, user) = setup();

    let result = create_task(&mut svm, &user, 1, &"a".repeat(281));

    assert_error(result, TodoError::DescriptionTooLong);
}

#[test]
fn completing_twice_fails() {
    let (mut svm, user) = setup();
    let task = task_pda(&user.pubkey(), 1);
    create_task(&mut svm, &user, 1, "My task").unwrap();
    complete_task(&mut svm, &user, task).unwrap();

    let result = complete_task(&mut svm, &user, task);

    assert_error(result, TodoError::AlreadyCompleted);
}
