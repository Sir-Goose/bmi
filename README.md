# BMI Calculator

A simple, static BMI calculator built with **Rust (Leptos 0.8)**, **Trunk**, and **Tailwind CSS v4**, compiled to WebAssembly. Deployable as a static site on Cloudflare Pages.

## Features

- Metric (cm / kg) and Imperial (ft+in / lbs) unit toggle with live conversion
- Reactive BMI calculation
- Color-coded category display (Underweight / Normal / Overweight / Obese)
- Visual gauge showing where your BMI falls on the spectrum
- ~190 KB total (156 KB WASM + JS glue + CSS)

## Prerequisites

### Local development

- [Rust](https://rustup.rs) (stable, 1.88+)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev): `cargo install trunk --locked`
- [Node.js](https://nodejs.org/) 18+ (for Tailwind CSS CLI)
- `npm install` to fetch Tailwind

### Quick start

```sh
npm install
trunk serve
```

Open http://127.0.0.1:3000 in your browser.

### Production build

```sh
trunk build --release
```

Output goes to `dist/`. Serve it with any static file server:

```sh
npx serve dist
# or
python3 -m http.server 8080 --directory dist
```

## Deploy to Cloudflare Pages

### Option A — Git-connected (recommended)

1. Push this repo to GitHub.
2. In the Cloudflare dashboard: **Workers & Pages → Create → Pages → Connect to Git**.
3. Select the repo and configure:
   - **Build command:** `./build.sh`
   - **Build output directory:** `dist`
   - **Environment variables (optional, for caching):**
     - `NODE_VERSION` = `20`
4. Click **Save and Deploy**.

The first build takes ~3–5 minutes (installs Rust + trunk). Subsequent builds are faster if you configure [build cache directories](https://developers.cloudflare.com/pages/configuration/build-caching/) in the dashboard:

- `~/.cargo/registry`
- `~/.cargo/git`
- `~/.rustup`
- `target`
- `node_modules`

### Option B — Wrangler CLI

```sh
npm install -g wrangler
trunk build --release
wrangler pages deploy dist --project-name bmi-calculator
```

## Tech stack

| Layer        | Tool                                      |
| ------------ | ----------------------------------------- |
| Framework    | [Leptos 0.8](https://leptos.dev) (CSR/WASM) |
| Bundler      | [Trunk 0.21](https://trunkrs.dev)         |
| Styling      | [Tailwind CSS v4](https://tailwindcss.com)|
| Deploy       | Cloudflare Pages (static)                 |

## Project structure

```
.
├── build.sh              # Cloudflare Pages build script
├── index.html            # Trunk entry point
├── Cargo.toml            # Rust dependencies
├── Trunk.toml            # Trunk config + Tailwind hook
├── package.json          # Tailwind CLI dependency
├── rust-toolchain.toml   # Pins Rust stable + wasm target
├── wrangler.toml         # Cloudflare wrangler config
├── src/
│   ├── main.rs           # Entry: mounts the app
│   └── app.rs            # BMI calculator component
└── styles/
    └── input.css         # Tailwind v4 source
```

## License

MIT
