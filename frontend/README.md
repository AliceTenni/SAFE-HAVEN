# 🖥️ SAFE-HAVEN — Frontend

React + TypeScript + Vite frontend for the [SAFE-HAVEN](../contracts/) Soroban smart contract on Stellar.

## Stack

| Layer | Technology |
|---|---|
| Framework | React 18 + TypeScript |
| Build tool | Vite 5 |
| Styling | Tailwind CSS 3 |
| Stellar SDK | `@stellar/stellar-sdk` v12 |
| Wallet | Freighter browser extension |
| Toasts | `react-hot-toast` |

---

## Features

| Feature | Description |
|---|---|
| 🔐 Wallet connect | Freighter wallet integration with session persistence |
| 📊 Dashboard | Live view of all your deposits with countdown timers |
| 💰 Deposit | Lock any SEP-41 token with custom unlock time and penalty |
| ⬆️ Withdraw | Claim unlocked tokens or cancel early with penalty |
| 🛡️ Admin panel | Pause/unpause + emergency withdrawal (admin only) |
| 🔗 Explorer links | Every address and tx links to Stellar Expert |

---

## Environment Variables

| Variable | Purpose | Required | Example |
|---|---|---|---|
| `VITE_CONTRACT_ID` | Deployed Soroban contract ID (from `stellar contract deploy`) | **Yes** | `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4` |
| `VITE_NETWORK_PASSPHRASE` | Stellar network passphrase | **Yes** | `Test SDF Network ; September 2015` |
| `VITE_RPC_URL` | Soroban RPC endpoint | **Yes** | `https://soroban-testnet.stellar.org` |
| `VITE_HORIZON_URL` | Horizon endpoint for account queries | **Yes** | `https://horizon-testnet.stellar.org` |
| `VITE_EXPLORER_URL` | Stellar Expert explorer base URL | **Yes** | `https://stellar.expert/explorer/testnet` |
| `VITE_SIMULATION_ACCOUNT` | Source account used for read-only simulations when no wallet is connected. Defaults to the contract ID if unset. Set to a funded account on your target network to avoid synthetic-account fallback. | No | `G...` (any funded Stellar account) |

---

## Getting Started

### 1. Prerequisites

- [Node.js 20+](https://nodejs.org/)
- [Freighter wallet](https://freighter.app) browser extension
- A deployed SAFE-HAVEN contract (see [`../contracts/`](../contracts/))

### 2. Install dependencies

```bash
cd frontend
npm install
```

### 3. Configure environment

```bash
cp .env.example .env
```

Edit `.env` and set at minimum:

```env
VITE_CONTRACT_ID=C...   # Your deployed contract address
```

### 4. Run dev server

```bash
npm run dev
```

Open [http://localhost:5173](http://localhost:5173).

### 5. Build for production

```bash
npm run build
npm run preview   # preview the production build locally
```

---

## Project Structure

```
src/
├── main.tsx              # React entry point
├── App.tsx               # Root component, tab routing
├── config.ts             # Contract ID, RPC URLs, constants
├── types.ts              # Shared TypeScript types
├── index.css             # Tailwind base + custom components
│
├── context/
│   └── WalletContext.tsx # Freighter wallet state + signing
│
├── hooks/
│   ├── useDeposits.ts    # Load deposits for connected address
│   └── useContractInfo.ts # Contract admin/paused/constants
│
├── lib/
│   ├── stellar.ts        # Contract reads + tx builders
│   └── format.ts         # Stroops, dates, countdown, BPS
│
├── components/
│   ├── Header.tsx        # Top nav + wallet button
│   ├── TabNav.tsx        # Page tab switcher
│   ├── DepositCard.tsx   # Single deposit UI card
│   └── TxStatusBadge.tsx # Signing → submitting → confirmed
│
└── pages/
    ├── Dashboard.tsx     # My vaults overview
    ├── DepositPage.tsx   # New deposit form
    ├── WithdrawPage.tsx  # Withdraw / cancel form
    └── AdminPage.tsx     # Admin controls
```

---

## Connecting to Testnet vs Mainnet

Change the values in `.env`:

| Variable | Testnet | Mainnet |
|---|---|---|
| `VITE_NETWORK_PASSPHRASE` | `Test SDF Network ; September 2015` | `Public Global Stellar Network ; September 2015` |
| `VITE_RPC_URL` | `https://soroban-testnet.stellar.org` | `https://soroban.stellar.org` |
| `VITE_HORIZON_URL` | `https://horizon-testnet.stellar.org` | `https://horizon.stellar.org` |
| `VITE_EXPLORER_URL` | `https://stellar.expert/explorer/testnet` | `https://stellar.expert/explorer/public` |

---

## Wallet Support

Currently the frontend integrates with **Freighter** directly via `window.freighter`.

To extend with more wallets (Albedo, xBull, Lobstr, etc.), the `WalletContext.tsx` signing logic can be replaced with `@creit.tech/stellar-wallets-kit` — the package is already included in `package.json`.
