# BagsTip contracts — task list

No Python venv: use **rustup** + **`rust-toolchain.toml`** + per-project **`Cargo.lock`** / `target/`.

GitHub: [bagstip-contracts issues #1–#3](https://github.com/BagsTip/bagstip-contracts/issues).

---

## Phase 0 — Toolchain (once per machine)

- [ ] Install [rustup](https://rustup.rs/) (Rust)
- [ ] Install [Solana CLI](https://docs.solanalabs.com/cli/install)
- [ ] Install [Anchor](https://www.anchor-lang.com/docs/installation) **0.31.1** (`cargo install anchor-cli --locked --version 0.31.1`)
- [ ] From repo root: `anchor build` succeeds (generates IDL under `target/idl/`)

---

## Phase 1 — Issue #1: TipVault MVP program

- [ ] Replace placeholder program id: `anchor keys list` / sync `declare_id!` + `Anchor.toml`
- [ ] Define accounts: escrow PDA (or vault), tip state (handle/claim id, tipper, amount, status)
- [ ] Instruction: **create_tip** — transfer SOL into escrow, init state (`pending`)
- [ ] Instruction: **release_tip** — verify creator (per MVP rules), send SOL to creator, set `claimed`
- [ ] Custom errors for bad state / wrong signer
- [ ] `anchor build` + fix clippy/fmt if you enable them

---

## Phase 2 — Issue #2: One e2e test (create → release)

- [ ] Add Anchor TS test harness (`package.json`, `tsconfig`, `tests/*.ts`) **or** Rust tests if you prefer
- [ ] Test: create tip → release → assert balances and/or account status
- [ ] Document in `README.md`: exact command to run tests (e.g. `anchor test`)

---

## Phase 3 — Issue #3: Devnet + IDL handoff

- [ ] `solana config set --url devnet` + wallet with SOL (airdrop)
- [ ] `anchor deploy` on devnet; record **program id**
- [ ] Commit or attach **`target/idl/bagstip_tipvault.json`** (or ensure reproducible path in docs)
- [ ] Extend `README.md`: deploy steps, cluster, program id location
- [ ] Ping backend owner with program id + IDL (Member B)

---

## Order (do not skip)

1. Phase 0  
2. Phase 1  
3. Phase 2  
4. Phase 3  

## Done when

- [ ] At least one automated test passes locally  
- [ ] Devnet program id + IDL available for API integration  
