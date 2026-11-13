# Case Board

A planning board for video editors: footage, references, assets, beats and exports pinned on one desk.

- **Website:** a Next.js app. The board lives in `public/board.html` and is served at `/`.
- **Windows app:** `desktop/` is a small Tauri app that opens the live website in its own window. Every deploy to the site reaches the app the next time it opens. The board also shows **New version ready** while it's open.

## Run the website

```bash
npm install
npm run dev        # http://localhost:3000
```

## Run the Windows app (on Windows)

You need Node.js and Rust (`winget install Rustlang.Rustup`).

```bash
cd desktop
npm install
npm run app        # opens the live site in the app window
npm run app:local  # opens http://localhost:3000 instead (run `npm run dev` in the root first)
```

## How updates ship

| What changed | What you do | Who gets it |
|---|---|---|
| The board (anything in `public/`, `app/`) | Merge to `main`. Vercel deploys. | Everyone, next time the app opens, or via the **New version ready** pill. |
| The Windows app itself (`desktop/`) | Push a tag like `v1.2.3`. The **Windows app** workflow builds `CaseBoard_1.2.3.0_x64.msix`. Upload it in Partner Center. | Everyone, through the Microsoft Store, after Microsoft's review. |

Each Store upload needs a higher version than the last one.

## Microsoft Store setup (once)

In Partner Center, open the app → **Product identity**, and copy these three values into
GitHub → **Settings → Secrets and variables → Actions → Variables**:

| Variable | Partner Center field |
|---|---|
| `STORE_IDENTITY_NAME` | Package/Identity/Name |
| `STORE_PUBLISHER` | Package/Identity/Publisher (starts with `CN=`) |
| `STORE_PUBLISHER_DISPLAY_NAME` | Package/Properties/PublisherDisplayName |

Until they're set, the workflow still builds, but the `.msix` uses test values and the Store will reject it.
