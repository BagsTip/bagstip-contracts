# BagsTip — V1 flow vetting (all repos)

**Command-center source of truth:** the **locked product flow** in §1. Everything below maps **four repos** to that flow: what exists, what’s wrong, what to add/remove/refactor.

**Repos in scope**

| Repo | Role |
|------|------|
| [bagstip-api](https://github.com/BagsTip/bagstip-api) | REST API, Supabase, custodial escrow transfers, X verification |
| [bagstip-web](https://github.com/BagsTip/bagstip-web) | Next.js dashboard / tip UI |
| [bagstip-bot](https://github.com/BagsTip/bagstip-bot) | X bot: mentions, replies, triggers |
| [bagstip-contracts](https://github.com/BagsTip/bagstip-contracts) | Anchor program (v2 / non-MVP payout track) |

Local reference clones (gitignored under this workspace): `bagstip-api/`, `bagstip-web/`, `bagstip-bot/`.

---

## 1. Locked V1 flow (source of truth)

### Money & custody

- **One custodial “vault” wallet** (single middleman). All tips that are not yet paid to creators **sit in this wallet’s SOL balance**.
- **Release** to creators is **API-driven**: backend signs **vault → creator** after rules pass (see [`MVP_ARCHITECTURE.md`](./MVP_ARCHITECTURE.md)).

### Identity model

- **Phantom (or any) wallet** can be linked to **exactly one X (Twitter) handle** for product purposes, with **one-time verification** (post code / bio check). After link, **no repeat verification** for that wallet↔X pair unless you explicitly reset.

### Tipper path (dashboard — “I am a tipper”)

1. Connect wallet.
2. Enter **their** X handle (the account they’ll tip from / identify with on X).
3. Dashboard shows a **verification code** and instructions to **post on X** (and/or put in bio) and optionally **share a dashboard link**.
4. Backend verifies the code appears **from that X username only** (not someone else impersonating).

### Creator path (dashboard — “I am a creator” / claim)

1. Connect wallet.
2. If **no X linked** to this wallet yet: same style **code verification** as tipper.
3. **Claim** pending SOL: flows that end in **`/claim/release`**-style payout after verification (aligned with API).

### Bot path (`@BagsTipBot` …)

- User posts e.g. `@BagsTipBot tip <amount> @<creator>`.
- Bot resolves **author’s X identity** and checks whether that X handle is **linked to a wallet** in your system.
- **Routing (intended):**
  - If **creator** has **not** completed dashboard link / payout setup: treat tip as **to vault** (pending for that creator) — *requires API + tx design*.
  - If **creator** is **linked/verified** as in product spec: **direct** payout path — *currently conflicts with single-vault custody unless “direct” means “API sends from vault to creator’s linked wallet immediately”* — clarify as **API instant release** vs **user-signed tx**.
- If **tipper** X is **not** linked to any wallet: bot tells them to **dashboard first** (link wallet + verify X).

### Notifications

- If creator **not** verified: bot **reply and/or DM** — “You received X SOL — claim at `bagstip.xyz/claim`” (exact copy TBD).

### History

- **Tipper dashboard:** list tips **they sent** — amount, **which creator** (handle), status.
- **Creator dashboard:** list tips **received** — amount, **who tipped** (handle and/or wallet if known), status.

---

## 2. Cross-cutting gaps (all repos)

| # | Gap | Severity |
|---|-----|----------|
| G1 | **Bot ↔ API** — bot code does **not** call the API; it uses **Anchor + bot wallet** as signer. V1 needs **`API_BASE_URL` + authenticated endpoints** (tip intent, link lookup, vault vs direct). | Blocker for unified V1 |
| G2 | **“Direct tip when creator linked”** vs **single vault** — must be one sentence in product: either “direct” = **API sends from vault to creator wallet in one step** (still custodial), or user signs another tx. Today API only **logs** tips with **`/tip/log`** after an existing **on-chain** tx. | Blocker |
| G3 | **Tipper X ↔ wallet** — schema/API has **no** first-class `tipper_profiles` (wallet + x_username + verified_at). Tips only store `tipper_wallet` on **`/tip/log`**. | High |
| G4 | **Web** — no wallet adapter, no verification UI, no `/dashboard`, no `/claim`, **fake** tip submit. | High |
| G5 | **Contracts** — Anchor program **not** used in MVP payout path per [`MVP_ARCHITECTURE.md`](./MVP_ARCHITECTURE.md); bot still references **program ID + IDL**. | Medium (confusion) |

---

## 3. Repo-by-repo report

### 3.1 `bagstip-api`

**What’s there**

- **Express** app: `POST /tip/log`, `GET /creator/:handle`, `POST /claim/init`, `/claim/verify`, `/claim/release` (`src/index.js`, `routes/*`).
- **Supabase** client (`src/db.js`); schema in `supabase_init.sql`: `tips`, `creators`, `claim_attempts`.
- **Custodial** `contract.js`: mock or `SystemProgram.transfer` from **`ESCROW_PRIVATE_KEY`**.
- **Verification** `verification.js`: mock or X API bio contains code (`X_BEARER_TOKEN`).

**Aligned with V1**

- Single vault / backend release model.
- Creator stats and claim pipeline skeleton.
- Username normalization (lowercase, strip `@`).

**Missing / wrong for V1**

- **No** `tipper` identity table (wallet + `x_username` + `verified_at` + `verification_code` flow for tipper-only).
- **`/tip/log`** requires **`txSig`** — fine for **web** after user sends SOL, **not** enough for **bot-only** intent unless you add **`POST /tip/bot-intent`** or extend contract.
- **No** endpoints for: “is X linked to wallet?”, “record bot-originated pending tip”, “tipper history by wallet”.
- **`.env.example`** incomplete vs code (`ESCROW_PRIVATE_KEY` vs `ESCROW_WALLET_ADDRESS` mismatch).
- **`bagstip.db` / sqlite** artifacts in repo if any — prefer **one** DB story (Supabase only).

**Add**

- Tables + routes: **tipper profile** (wallet pubkey unique, x handle, verified flag, timestamps).
- **Bot-facing** routes (shared secret header): resolve **X user → wallets**, create **pending tip** rows when vault receives or when bot records intent (design with G2).
- **`GET /tipper/:wallet`** or **`GET /dashboard/tipper`** — outgoing tips for connected wallet.
- Harden **`.env.example`**: `RPC_URL`, `ESCROW_PRIVATE_KEY`, `ESCROW_WALLET_ADDRESS`, `CONTRACT_MODE`, Supabase keys.

**Remove / stop**

- Committing local **SQLite** DB files if production uses Supabase only.

**Refactor**

- Single **service layer** for “who can receive payout” vs “pending tips” to avoid duplicating queries across claim + future bot.

---

### 3.2 `bagstip-web`

**What’s there**

- **Next.js 16** App Router, Tailwind, **one** route **`/tip/[handle]`** with **`ClientTipForm`**: tweet URL + amount + **mock** success (`setTimeout`).
- **Home** redirects to **`/tip/elon`** (`src/app/page.tsx`).
- **No** `@solana/wallet-adapter`, **no** API calls, **no** claim/dashboard routes.

**Aligned with V1**

- Visual direction for a tip page only.

**Missing for V1**

- **Entire** dashboard: tipper vs creator **modes**, wallet connect, X handle field, **code display**, poll or “verify” button calling API.
- **Real** tip: build **SOL transfer** to **vault pubkey** from connected wallet, then **`POST /tip/log`** with **`txSig`**.
- **`/claim`** (or embedded creator flow) wired to **`/claim/*`** API.
- **History** UIs for both roles.

**Add**

- Dependencies: **`@solana/wallet-adapter-*`**, **`@solana/web3.js`**, **`fetch` to API**.
- Routes: e.g. `/dashboard`, `/dashboard/tipper`, `/dashboard/creator`, `/claim`, **`/verify`** or steps on dashboard.
- **Env:** `NEXT_PUBLIC_API_URL`, `NEXT_PUBLIC_VAULT_ADDRESS`, `NEXT_PUBLIC_SOLANA_NETWORK`.

**Remove**

- **`postbuild`** git commit hook in `package.json` (surprising in CI and unsafe) — replace with normal CI or remove.

**Refactor**

- Replace hardcoded **`/tip/elon`** redirect with **marketing home** or **role picker**.

---

### 3.3 `bagstip-bot`

**What’s there**

- **`bot.ts`**: polls mentions (incomplete — **`userMentions`** needs **user id** in real v2 API), regex **`tip @handle amount`**, then **`@coral-xyz/anchor`** + **hardcoded mini-IDL** calling **`program.methods.tip(...)`** with **`BOT_PRIVATE_KEY`** as signer — **not** the custodial vault model and **not** the API.
- **`implementation.md`**: old “no backend” recipe.
- **`package.json`**: `ts-node`, anchor, twitter-api-v2.

**Aligned with V1**

- Mentions loop + reply idea only.

**Wrong for V1 (must change)**

- **Bot must not** be the only signer of user tips; **user** tips from X should tie to **tipper’s linked wallet**, not **`BOT_PRIVATE_KEY`** funding everything.
- **No** Anchor calls for MVP vault (per [`MVP_ARCHITECTURE.md`](./MVP_ARCHITECTURE.md)).

**Add**

- **`process.env.API_BASE_URL`**, **`BOT_API_SECRET`** (or similar).
- After parsing mention: **`GET`** profile checks / **`POST`** tip-intent endpoints (once API exists).
- **Persist `last_tweet_id`** to file + Railway volume (as your spec).
- **403** duplicate tweet: swallow.
- **`node-cron`** or keep `setInterval` — align with ops.

**Remove**

- **Anchor** `Program` construction and **`PROGRAM_ID`** from MVP path (or gate behind `USE_ANCHOR=false`).

**Refactor**

- **Regex** to match agreed copy: `@BagsTipBot tip <amount> @creator` (and SOL unit if required).
- Use **Twitter API v2** correctly: **mentions timeline for bot user id** (`client.v2.userMentionTimeline` or search — fix the current API usage).

---

### 3.4 `bagstip-contracts` (this repo)

**What’s there**

- Anchor **0.31.1** program **`bagstip_tipvault`**, stub **`initialize`**, devnet program id documented.
- Docs: **`MVP_ARCHITECTURE.md`**, **`CONTRACT_DECISIONS_v1.md`**, **`TASKLIST.md`**.

**Aligned with V1**

- **Explicitly out of MVP hot path** for custodial release.

**For V1 product**

- **No** required change for launch if escrow stays off-chain.

**Add (optional)**

- **`docs/V1_VETTING.md`** (this file) — **keep updated** when API/bot/web ship.

**Remove / defer**

- Marketing **contracts** as **blocking** MVP — they’re **v2**.

**Refactor (later)**

- When moving to on-chain escrow: implement **PDAs / release** per **`CONTRACT_DECISIONS_v1.md`**, not the old bot fake IDL.

---

## 4. Suggested build order (execution)

1. **API** — tipper profile + bot endpoints + clarify G2 with one paragraph in this doc.
2. **Web** — wallet + vault transfer + `/tip/log` + dashboard verification UI + history reads.
3. **Bot** — strip Anchor, call API, persistence file, Railway.
4. **Contracts** — only if/when v2 on-chain escrow is scheduled.

---

## 5. One-line owner map

| Area | Suggested owner |
|------|------------------|
| Vault key + env + Supabase schema | Backend (API) |
| Dashboard + wallet + tx + history | Frontend (web) |
| X mentions + replies + API glue | Bot |
| Anchor program | Contracts (post-MVP) |

---

*Last updated: generated as V1 vetting snapshot; amend in PRs as implementation lands.*
