# Copy-paste: PR #4 review — [bagstip-bot](https://github.com/BagsTip/bagstip-bot/pull/4)

Use as **General** discussion comment first, then **inline / file** comments as needed.

---

## 1. Top-level PR comment (Conversation)

**Summary — direction LGTM, merge gated on API + small fixes**

This PR correctly moves the bot to V1: **no Anchor / no on-chain signing in the bot**, structured modules (`api`, `parser`, `logic`, `notifier`, `poller`), mention timeline + `last_tweet_id` persistence, and handling for `released` / `pending` / `insufficient_balance`. Removing the stray `lib.rs` and old `bot.ts` is right.

**Before merge:**

1. **Backend** — `GET …/profile/resolve` and `POST …/tip/bot-intent` must exist and match what `src/api.ts` calls; confirm whether `API_BASE_URL` includes `/api/v1` or paths need updating.
2. **`client.v2.reply`** — verify argument order against `twitter-api-v2` (reply to tweet id vs text).
3. **API response shape** — `logic.ts` expects `tipper.verified`; align with actual JSON from `/profile/resolve`.
4. **Solscan** — use full `https://` URLs and `?cluster=devnet` for devnet txs in replies.
5. **Username compare** — normalize case when skipping bot self (`X_BOT_HANDLE` vs `author.username`).

Detailed notes are in per-file comments below.

---

## 2. `src/api.ts`

**Suggested inline comment:**

> `fetch` to `/profile/resolve` and `/tip/bot-intent` assume `API_BASE_URL` is the full prefix (e.g. `https://xxx.railway.app/api/v1`). If the API mounts only at `/`, update `BASE` or path strings. Consider typed return types instead of bare `Promise` for easier alignment with backend JSON.

---

## 3. `src/logic.ts`

**Suggested inline comment:**

> `resolveProfile` — confirm backend returns an object where **`verified`** is the field name (vs `found` + nested profile). Also handle **`insufficient_balance`** if API returns it as HTTP 4xx with a body instead of `result.error` so we don’t show a generic failure.

**Second comment (released branch):**

> Solscan link: use `https://solscan.io/tx/${sig}?cluster=devnet` (or mainnet) so links work from X.

---

## 4. `src/notifier.ts`

**Suggested inline comment:**

> Please verify **`client.v2.reply`** signature for our `twitter-api-v2` version — some versions use `(replyToTweetId, text)` not `(text, id)`. Wrong order = failed replies.

---

## 5. `src/parser.ts`

**Suggested inline comment:**

> Regex hardcodes `@bagstipbot`. Lowercasing `text` first makes this OK; optional improvement: build the pattern from `process.env.X_BOT_HANDLE` so renames don’t require code change.

---

## 6. `src/poller.ts`

**Suggested inline comment:**

> Self-skip: compare `author.username.toLowerCase()` with `process.env.X_BOT_HANDLE?.toLowerCase()` so case never lets bot reply to itself.

**Second comment:**

> After processing a batch, we `saveLastId(tweets[0].id)` — confirm Twitter returns tweets in **newest-first** order so `tweets[0]` is the max id (cursor). If API order differs, use `Math.max` over ids.

---

## 7. `src/index.ts`

**Suggested inline comment:**

> `setInterval` + async `pollMentions` can overlap if a poll runs longer than `POLL_INTERVAL_MS`. Optional: guard with a `polling` boolean or use a queue so two polls never run concurrently.

---

## 8. `package.json` / build

**Suggested inline comment:**

> `npm run build` + `node dist/index.js` is good for Railway. Document **Node ≥18** (global `fetch`) in README if not already.

---

## 9. `implementation.md`

**Suggested inline comment:**

> This file now duplicates the team spec. Consider trimming to **env + runbook** only and linking to the canonical doc in the org to avoid drift.

---

## 10. PR metadata

**Suggested inline comment (Files changed / overview):**

> PR merges **`main` → `develop`**. Confirm that’s intentional for branch strategy; if the team expects feature branches into `main`, adjust for next PRs.

---

*Generated for BagsTip PR #4 — commit `764c635`.*
