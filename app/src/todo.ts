import {
  appendTransactionMessageInstruction,
  createSolanaRpc,
  createTransactionMessage,
  getBase58Decoder,
  getBase64Encoder,
  pipe,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signAndSendTransactionMessageWithSigners,
  type Address,
  type Base58EncodedBytes,
  type Signature,
  type Instruction,
  type TransactionSendingSigner,
} from "@solana/kit";
import {
  findTaskPda,
  getCompleteTaskInstruction,
  getCreateTaskInstruction,
  getDeleteTaskInstruction,
  getTaskDecoder,
  getUpdateTaskInstruction,
  TASK_DISCRIMINATOR,
  TODO_PROGRAM_ADDRESS,
} from "./generated";

export const RPC_URL = import.meta.env.VITE_RPC_URL ?? "https://api.devnet.solana.com";
export const rpc = createSolanaRpc(RPC_URL);

export async function fetchBalance(owner: Address) {
  const { value } = await rpc.getBalance(owner).send();
  return value;
}

// Builds a transaction with one instruction, lets the wallet sign and send it,
// and waits until the network confirms it.
async function send(signer: TransactionSendingSigner, instruction: Instruction) {
  const { value: blockhash } = await rpc.getLatestBlockhash().send();
  const message = pipe(
    createTransactionMessage({ version: 0 }),
    (m) => setTransactionMessageFeePayerSigner(signer, m),
    (m) => setTransactionMessageLifetimeUsingBlockhash(blockhash, m),
    (m) => appendTransactionMessageInstruction(instruction, m),
  );
  const signature = getBase58Decoder().decode(
    await signAndSendTransactionMessageWithSigners(message),
  ) as Signature;
  for (let i = 0; i < 60; i++) {
    const { value } = await rpc.getSignatureStatuses([signature]).send();
    if (value[0]?.err) throw new Error(JSON.stringify(value[0].err));
    if (value[0]?.confirmationStatus === "confirmed") return signature;
    if (value[0]?.confirmationStatus === "finalized") return signature;
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`Transaction not confirmed: ${signature}`);
}

export async function fetchTasks(owner: Address) {
  const discriminator = getBase58Decoder().decode(TASK_DISCRIMINATOR);
  const accounts = await rpc
    .getProgramAccounts(TODO_PROGRAM_ADDRESS, {
      encoding: "base64",
      filters: [
        { memcmp: { offset: 0n, bytes: discriminator as Base58EncodedBytes, encoding: "base58" } },
        {
          memcmp: { offset: 8n, bytes: owner as string as Base58EncodedBytes, encoding: "base58" },
        },
      ],
    })
    .send();
  return accounts.map(({ pubkey, account }) => ({
    address: pubkey,
    ...getTaskDecoder().decode(getBase64Encoder().encode(account.data[0])),
  }));
}

export async function createTask(signer: TransactionSendingSigner, description: string) {
  const taskId = BigInt(Date.now());
  const [task] = await findTaskPda({ owner: signer.address, taskId });
  const instruction = getCreateTaskInstruction({ owner: signer, task, taskId, description });
  return send(signer, instruction);
}

export function updateTask(signer: TransactionSendingSigner, task: Address, description: string) {
  return send(signer, getUpdateTaskInstruction({ owner: signer, task, description }));
}

export function completeTask(signer: TransactionSendingSigner, task: Address) {
  return send(signer, getCompleteTaskInstruction({ owner: signer, task }));
}

export function deleteTask(signer: TransactionSendingSigner, task: Address) {
  return send(signer, getDeleteTaskInstruction({ owner: signer, task }));
}
