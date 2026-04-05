# BagsTip contracts — task list

Tracks [GitHub issues](https://github.com/BagsTip/bagstip-contracts/issues) for the TipVault MVP.

## Issue #1 — [MVP] TipVault contract ship checklist

- [ ] Anchor program crate builds (`anchor build`)
- [ ] Accept tip into escrow (SOL path; token path only if required)
- [ ] Store creator handle / claim id, tipper, amount, status on-chain or in PDA state
- [ ] Instruction: release funds to verified creator wallet
- [ ] Mark tip claimed after successful release
- [ ] Error handling for invalid state transitions
- [ ] Devnet deploy (see #3) + IDL export for backend

## Issue #2 — Basic test: create tip → release

- [ ] `tests/` (TS) or `tests/*.rs`: one flow covering create → release
- [ ] Assertions on final status / balances as appropriate
- [ ] Document test command in `README.md` (e.g. `anchor test`)

## Issue #3 — Devnet deploy + share IDL

- [ ] Configure wallet + cluster for devnet (`solana config set --url devnet`)
- [ ] Fund deployer (faucet)
- [ ] `anchor deploy` (or documented equivalent) → record **program id**
- [ ] Export IDL: `target/idl/bagstip_tipvault.json` (or generated name) committed or linked
- [ ] Document deploy steps + env vars in `README.md`
- [ ] Notify backend owner (Member B) with program id + IDL path

## Dependency order

1. #1 scaffold + core instructions  
2. #2 tests on localnet  
3. #3 devnet + IDL handoff  

## Done when

- Backend can integrate against a real devnet IDL and program id, and at least one automated test passes locally.
