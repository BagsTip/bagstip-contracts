# bagstip-contracts

Solana programs for [BagsTip](https://github.com/BagsTip) — MVP **TipVault** escrow (tips in, release to verified creator).

## Prerequisites

Install on your machine (not bundled with this repo):

1. [Rust](https://www.rust-lang.org/tools/install) (see `rust-toolchain.toml` for pinned channel)
2. [Solana CLI](https://docs.solanalabs.com/cli/install)
3. [Anchor](https://www.anchor-lang.com/docs/installation) **0.30.x** (match `Anchor.toml`)

## Quick start

```bash
cd bagstip-contracts
anchor build
```

IDL output: `target/idl/bagstip_tipvault.json` (after a successful build).

### Tests

TypeScript tests (once `package.json` / `tests/` are added):

```bash
yarn install
anchor test
```

Until then, follow `TASKLIST.md` to add the first test suite.

### Devnet deploy

```bash
solana config set --url devnet
solana airdrop 2   # if needed
anchor deploy
```

Record the **program id** from `Anchor.toml` / deploy output and share the IDL with the API owner.

## Work tracking

See **`TASKLIST.md`** for issue-aligned checklists ([#1](https://github.com/BagsTip/bagstip-contracts/issues/1)–[#3](https://github.com/BagsTip/bagstip-contracts/issues/3)).

## Program id

The repo uses a **placeholder** program id in `programs/tipvault/src/lib.rs` and `Anchor.toml`. Before mainnet, run `anchor keys list` / `anchor keys sync` so the declared id matches your keypair.
