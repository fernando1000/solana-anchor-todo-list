import {
  useConnect,
  useWallets,
  type UiWallet,
  type UiWalletAccount,
} from "@wallet-standard/react";

type OnConnect = (account: UiWalletAccount) => void;

// Every installed wallet that follows the Wallet Standard and can send
// Solana transactions shows up here, Phantom included.
export function ConnectButtons({ onConnect }: { onConnect: OnConnect }) {
  const wallets = useWallets().filter((wallet) =>
    wallet.features.includes("solana:signAndSendTransaction"),
  );
  if (wallets.length === 0) return <p className="hint">No Solana wallet found.</p>;
  return wallets.map((wallet) => (
    <ConnectButton key={wallet.name} wallet={wallet} onConnect={onConnect} />
  ));
}

function ConnectButton({ wallet, onConnect }: { wallet: UiWallet; onConnect: OnConnect }) {
  const [isConnecting, connect] = useConnect(wallet);
  async function handleClick() {
    const [account] = await connect();
    if (account) onConnect(account);
  }
  return (
    <button className="primary" disabled={isConnecting} onClick={handleClick}>
      <img src={wallet.icon} alt="" width={18} height={18} />
      Connect {wallet.name}
    </button>
  );
}
