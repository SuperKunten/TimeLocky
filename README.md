# ⏳ TimeLocky

**Solana SPL Token Time Lock & Vesting** — and more features are coming.

A desktop GUI for creating and managing on-chain timelocks, vesting schedules, and payment streams on Solana. Built with **Python** (CustomTkinter) + **Rust** (PyO3 for fast Borsh serialization).

![Python](https://img.shields.io/badge/Python-3.10+-blue) ![Rust](https://img.shields.io/badge/Rust-1.70+-orange) ![Solana](https://img.shields.io/badge/Solana-Devnet%2FMainnet-purple)

---

## 🚀 Getting Started

For a detailed installation and usage guide, please see [GUIDE.md](GUIDE.md).

### 📋 Prerequisites

-   🐍 **Python 3.10+**
-   🦀 **Rust toolchain** (optional, for the compiled extension) — [Install Rust](https://rustup.rs)
-   🔑 A Solana keypair file (`.json`) or base58 private key

### 🛠️ Installation

```bash
cd "Timelocky"
python setup.py          # installs deps + builds Rust extension
```

Or manually:

```bash
pip install -r requirements.txt
pip install maturin && maturin develop --release   # optional Rust build
```

### ▶️ Run

```bash
python main.py
```

### 🎉 First launch

1.  **✍️ Register** — copy your HWID from the gate screen and send it to the admin
2.  **👛 Connect wallet** — go to 💰 WALLET, load your `.json` keypair or paste a base58 key
3.  **🌐 Pick cluster** — devnet for testing (free airdrops), mainnet for real tokens
4.  **🔓 Create a stream** — go to 🔓 VESTING, fill in recipient, token mint, amount, and schedule
5.  **⛓️ Manage** — go to ⛓️ MANAGE to withdraw, cancel, or top-up active streams

> **💡 Tip:** The app works without Rust — it falls back to pure-Python serialization. The top bar shows whether `RUST_CORE` is loaded.

---

## ✨ Features

- ⏳ **Token Vesting & Locking** — schedule token releases over custom periods with cliff support
- 🔀 **Mix Payouts** — randomize vesting amounts per period for natural distributions
- 🗓️ **Choose Periods** — flexible unlock schedules (minutes, hours, days, weeks, months)
- 💰 **Wallet Manager** — load keypair from file or base58 key, check balance, request devnet airdrops
- ⛓️ **Stream Manager** — view, withdraw, cancel, and top-up active streams
- 📈 **Statistics** — track vesting progress with summary metrics and per-stream detail cards
- 📖 **Address Book** — save & reuse recipient wallet addresses
- 🔒 **HWID Registration** — machine-locked access with HMAC-SHA3-512 fingerprinting
- 🔄 **Auto-Updater** — checks GitHub Releases for new versions on startup
- 🧬 **Rust Core** — fast Borsh serialization via PyO3 (with pure-Python fallback)
- 🌐 **Multi-Cluster** — switch between devnet, testnet, and mainnet

## 🏛️ Architecture

```
TimeLocky/
├── main.py                          # Entry point
├── hwid_tool.py                     # Standalone HWID generator GUI
├── setup.py                         # Build script
├── pyproject.toml                   # Maturin + project config
├── requirements.txt                 # Python dependencies
├── Cargo.toml                       # Rust workspace
├── rust_core/                       # Rust PyO3 extension
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                   # PyO3 bindings + HMAC secret
│       ├── models.rs                # Borsh data structures
│       └── timelocky.rs             # Instruction builders
└── gui/                             # Python GUI application
    ├── app.py                       # Main app (registration gate + updater)
    ├── theme.py                     # CYBER dark theme
    ├── utils/
    │   ├── app_config.py            # Config persistence + version
    │   ├── hwid.py                  # HMAC-SHA3-512 fingerprinting
    │   ├── registration_guard.py    # @registration_required decorator
    │   ├── updater.py               # GitHub Releases auto-updater
    │   ├── solana_client.py         # Solana RPC wrapper
    │   ├── timelocky_client.py      # On-chain timelock client
    │   ├── address_book.py          # Saved wallets
    │   └── wallet_config.py         # Wallet persistence
    └── views/
        ├── main_view.py             # Dashboard
        ├── wallet_view.py           # Wallet connection
        ├── create_stream.py         # Vesting/locking form
        ├── manage_streams.py        # Stream management
        ├── statistics_view.py       # Analytics
        ├── address_book_view.py     # Address book UI
        └── registration_view.py     # HWID registration gate
```


## Stream Parameters

| Parameter | Description |
|-----------|-------------|
| `start_time` | When the stream begins (unix timestamp) |
| `net_amount_deposited` | Total tokens to vest/lock |
| `period` | Release interval in seconds |
| `amount_per_period` | Tokens released each period |
| `cliff` | Cliff timestamp (initial lock period) |
| `cliff_amount` | Tokens released at cliff |
| `cancelable_by_sender` | Sender can cancel the stream |
| `cancelable_by_recipient` | Recipient can cancel |
| `automatic_withdrawal` | Auto-send to recipient |
| `transferable_by_sender` | Sender can reassign |
| `transferable_by_recipient` | Recipient can reassign |
| `can_topup` | Allow adding more tokens |

## Security

- 🔐 Private keys are **never stored or logged** — all signing happens locally
- 🔒 HWID registration uses **HMAC-SHA3-512** with a secret baked into the compiled Rust binary
- ✅ All client methods are gated with `@registration_required` decorators
- 🚫 Banned accounts (`valid: false`) are blocked at startup

## License

MIT
