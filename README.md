⏳ TimeLocky — Solana Token Vesting & Time-Locking Made Easy
Ready to dive into the world of Solana token time-locking and vesting? TimeLocky is a professional-grade desktop application that makes creating and managing on-chain timelocks, vesting schedules, and payment streams on Solana as smooth as possible. Built with a sleek cyberpunk-themed interface powered by CustomTkinter and backed by a blazing-fast Rust core via PyO3, TimeLocky empowers token creators, DAOs, and project teams to lock SPL tokens with fully customizable release schedules — all executed trustlessly on-chain.

TimeLocky supports two primary modes: Vesting (gradual token release over time) and Locking (single unlock after a set period). You can configure granular parameters like start time, unlock frequency (from seconds to years), cliff amounts, cancellation permissions, automatic withdrawals, and top-up capability. A unique 🔀 Mix Payouts feature randomizes vesting amounts per period for natural-looking distributions.

Here's a quick tour of what you can do:

👛 Wallet Central — Hook up your Solana wallet, check balances, and request devnet airdrops
⏳ Create Time Locks — Lock up your SPL tokens and set them to release whenever you want
⛓️ Manage Streams — Withdraw, cancel, or top-up all your active vesting schedules
📊 Stat-o-Matic — Get cool charts and numbers about your locked assets
📓 Address Book — Your personal rolodex for all your Solana contacts
🌐 Multi-Cluster — Switch between devnet (free testing!), testnet, and mainnet

Security is a top priority — private keys are never stored or logged, all signing happens locally, and wallet credentials are encrypted at rest with Fernet (AES-128-CBC). Access is gated via HWID registration using HMAC-SHA3-512 fingerprinting. The app works without Rust (it falls back to pure-Python serialization), but it's definitely zippier with Rust!

✅ What You'll Need
🐍 Python 3.10+ dependencies:
solana>=0.34.0 · solders>=0.21.0 · base58>=2.1.0 · requests>=2.31.0 · customtkinter>=5.2.0 · Pillow>=10.0.0 · cryptography>=41.0.0 · maturin>=1.5.0

🦀 Rust 1.70+ (optional): pyo3, borsh, sha2, base64, serde, bs58

📬 Links & Contact
Telegram Channel: t.me/timelocky
Developer: @bandokaay_ofb
GitHub: SuperKunten/TimeLocky
