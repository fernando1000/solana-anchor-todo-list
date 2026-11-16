import { useCallback, useEffect, useState } from "react";
import type { Address } from "@solana/kit";
import { useWalletAccountTransactionSendingSigner } from "@solana/react";
import type { UiWalletAccount } from "@wallet-standard/react";
import { completeTask, createTask, deleteTask, fetchBalance, fetchTasks, updateTask } from "./todo";

type TaskItem = Awaited<ReturnType<typeof fetchTasks>>[number];

const sol = (lamports: bigint) => (Number(lamports) / 1e9).toFixed(6);

export function TodoList({ account }: { account: UiWalletAccount }) {
  const signer = useWalletAccountTransactionSendingSigner(account, "solana:devnet");
  const owner = account.address as Address;

  const [tasks, setTasks] = useState<TaskItem[]>([]);
  const [balance, setBalance] = useState<bigint | null>(null);
  const [text, setText] = useState("");
  const [editing, setEditing] = useState<{ task: Address; text: string } | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    const [list, lamports] = await Promise.all([fetchTasks(owner), fetchBalance(owner)]);
    setTasks(list.sort((a, b) => Number(a.taskId - b.taskId)));
    setBalance(lamports);
  }, [owner]);

  useEffect(() => {
    refresh().catch((e) => setError(String(e)));
  }, [refresh]);

  async function run(action: () => Promise<unknown>) {
    setBusy(true);
    setError(null);
    try {
      await action();
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <section className="card">
      <div className="balance">
        <span className="label">Balance</span>
        <span className="value">{balance === null ? "…" : `${sol(balance)} SOL`}</span>
      </div>

      <form
        className="new-task"
        onSubmit={(e) => {
          e.preventDefault();
          if (!text.trim()) return;
          run(async () => {
            await createTask(signer, text.trim());
            setText("");
          });
        }}
      >
        <input
          value={text}
          maxLength={280}
          placeholder="What needs to be done?"
          onChange={(e) => setText(e.target.value)}
          disabled={busy}
        />
        <button className="primary" disabled={busy || !text.trim()}>
          Add task
        </button>
      </form>

      {error && <p className="error">{error}</p>}

      <ul className="tasks">
        {tasks.length === 0 && <li className="empty">No tasks yet.</li>}
        {tasks.map((task) => (
          <li key={task.address} className={task.completed ? "done" : ""}>
            {editing?.task === task.address ? (
              <form
                className="edit"
                onSubmit={(e) => {
                  e.preventDefault();
                  run(async () => {
                    await updateTask(signer, task.address, editing.text.trim());
                    setEditing(null);
                  });
                }}
              >
                <input
                  autoFocus
                  value={editing.text}
                  maxLength={280}
                  onChange={(e) => setEditing({ task: task.address, text: e.target.value })}
                  disabled={busy}
                />
                <button className="primary" disabled={busy || !editing.text.trim()}>
                  Save
                </button>
              </form>
            ) : (
              <>
                <span className="description">{task.description}</span>
                <div className="actions">
                  {!task.completed && (
                    <>
                      <button
                        disabled={busy}
                        onClick={() => setEditing({ task: task.address, text: task.description })}
                      >
                        Edit
                      </button>
                      <button
                        disabled={busy}
                        onClick={() => run(() => completeTask(signer, task.address))}
                      >
                        Done
                      </button>
                    </>
                  )}
                  <button
                    className="danger"
                    disabled={busy}
                    onClick={() => run(() => deleteTask(signer, task.address))}
                  >
                    Delete
                  </button>
                </div>
              </>
            )}
          </li>
        ))}
      </ul>
    </section>
  );
}
