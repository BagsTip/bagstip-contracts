# bagstip-contracts

Solana programs for [BagsTip](https://github.com/BagsTip) — MVP **TipVault** escrow (tips in, release to verified creator).

## Prerequisites

Rust/Anchor do not use a Python-style venv: use **rustup** + this repo’s **`rust-toolchain.toml`** and **`Cargo.lock`**.

Install on your machine (not bundled with this repo):

1. [Rust](https://www.rust-lang.org/tools/install) (see `rust-toolchain.toml` for pinned channel)
2. [Solana CLI](https://docs.solanalabs.com/cli/install)
3. [Anchor](https://www.anchor-lang.com/docs/installation) **0.31.1** (match `Anchor.toml`; `cargo install anchor-cli --locked --version 0.31.1`)

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
# or:
# solana program deploy target/deploy/bagstip_tipvault.so --program-id target/deploy/bagstip_tipvault-keypair.json
```

**Current devnet program id:** `873HALYp7gZx6tDzPFoxa5UrA7uRJ2eafmCbqwHxSKFH`

Example deploy tx (devnet): `5KmzeAQjiEyqDm9LKabMYJFx3i1qazhvwghkRsRdswfVjPkVSs1UwgVzXqeVQ3nHcF7RryCex1sV6ERukQdxX9P9`

Share **`target/idl/bagstip_tipvault.json`** + program id with the API owner. Do **not** commit `*-keypair.json` (kept under `target/`, which is gitignored).

## Work tracking

See **`TASKLIST.md`** for issue-aligned checklists ([#1](https://github.com/BagsTip/bagstip-contracts/issues/1)–[#3](https://github.com/BagsTip/bagstip-contracts/issues/3)).

## Program id

Declared in `programs/tipvault/src/lib.rs` and `Anchor.toml` (`[programs.localnet]` / `[programs.devnet]`). Run `anchor keys sync` after rotating the program keypair.
