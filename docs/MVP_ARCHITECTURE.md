# BagsTip — MVP architecture (revised, hackathon)

**Status:** Current product decision for **MVP**. Supersedes on-chain–first flow for the hackathon window.

**Related:** Anchor / program notes remain in [`CONTRACT_DECISIONS_v1.md`](./CONTRACT_DECISIONS_v1.md) for **post-hackathon v2**.

---

## Summary

| Layer | Choice |
|--------|--------|
| **Escrow** | **Custodial** — one backend-controlled keypair (`ESCROW_PRIVATE_KEY` or equivalent in env). |
| **Tip (user → escrow)** | **Plain SOL transfer** — user wallet sends to escrow **pubkey**. **No Anchor**, no program ID on client for this path. |
| **Release (escrow → creator)** | **Backend signs** `SystemProgram.transfer` (or equivalent) **after** API gates pass. Matches existing [`bagstip-api`](https://github.com/BagsTip/bagstip-api) `contract.js` live sketch. |
| **Anchor / IDL** | **Not required** on client or server for MVP. |
| **v2** | Migrate **release** to **Anchor** with **creator signature** on-chain; optional keep custodial tip-in until then. |

---

## Flows

1. **Tip:** User wallet → escrow address (standard Solana transfer). API records tip after confirmation (existing patterns).
2. **Release:** After **API `verified = true`** (e.g. tweet check) and **creator wallet matches** the address stored for that verified creator, **backend** transfers **escrow → creator** using the escrow keypair.

---

## Security gates (release)

- **API `verified = true`** (off-chain verification pipeline).
- **Creator wallet** in DB must be the **only** payout target the backend uses for that claim (address the creator proved control of).

**Operational note:** Protect `ESCROW_PRIVATE_KEY` like production secrets (rotation, KMS in real deploy).

---

## Contracts repo

The **Anchor program** in this repository remains a **v2 / reference** track; MVP does **not** depend on deploying it for payout logic.
