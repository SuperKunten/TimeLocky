<p align="center">
  <img src="https://img.shields.io/badge/Solana-Token%20Vesting-9945FF?style=for-the-badge&logo=solana&logoColor=white" />
  <img src="https://img.shields.io/badge/Built%20With-Python%20%2B%20Rust-informational?style=for-the-badge&logo=python&logoColor=white" />
  <img src="https://img.shields.io/badge/UI-CustomTkinter-blue?style=for-the-badge" />
</p>

<h1 align="center"><a href="https://github.com/SuperKunten/TimeLocky/">⏳ TimeLocky</a></h1>
<h3 align="center">Solana Token Vesting & Time-Locking Made Easy</h3>

<p align="center">
  A professional-grade desktop application for creating and managing on-chain timelocks,<br>
  vesting schedules, and payment streams on <strong>Solana</strong>.
</p>

<p align="center">
  <a href="https://t.me/timelocky">Telegram</a> •
  <a href="https://github.com/SuperKunten/TimeLocky">GitHub</a>
</p>

---

## 🚀 Overview

**TimeLocky** empowers token creators, DAOs, and project teams to lock SPL tokens with fully customizable release schedules — all executed trustlessly on-chain.

Built with a sleek **cyberpunk-themed interface** powered by [CustomTkinter](https://github.com/TomSchimansky/CustomTkinter) and backed by a blazing-fast **Rust core** via [PyO3](https://pyo3.rs/), TimeLocky makes token vesting and locking as smooth as possible.

---

## 🔑 Modes

| Mode | Description |
|------|-------------|
| **Vesting** | Gradual token release over time with configurable frequency |
| **Locking** | Single unlock after a set period |

Configure granular parameters including start time, unlock frequency (seconds → years), cliff amounts, cancellation permissions, automatic withdrawals, and top-up capability.

> 🔀 **Mix Payouts** — Randomizes vesting amounts per period for natural-looking distributions.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 👛 **Wallet Central** | Connect your Solana wallet, check balances, and request devnet airdrops |
| ⏳ **Create Time Locks** | Lock SPL tokens and set custom release schedules |
| ⛓️ **Manage Streams** | Withdraw, cancel, or top-up active vesting schedules |
| 📊 **Stat-o-Matic** | Charts and analytics for your locked assets |
| 📓 **Address Book** | Personal rolodex for all your Solana contacts |
| 🌐 **Multi-Cluster** | Switch between **devnet**, **testnet**, and **mainnet** seamlessly |

---

## 🔒 Security

- **Private keys are never stored or logged**
- All signing happens **locally**
- Wallet credentials encrypted at rest with **Fernet (AES-128-CBC)**
- Access gated via **HWID registration** using **HMAC-SHA3-512** fingerprinting

---

## 📦 Requirements

### 🐍 Python 3.10+

```
solana >= 0.34.0
solders >= 0.21.0
base58 >= 2.1.0
requests >= 2.31.0
customtkinter >= 5.2.0
Pillow >= 10.0.0
cryptography >= 41.0.0
maturin >= 1.5.0
```

### 🦀 Rust 1.70+ *(optional)*

```
pyo3 · borsh · sha2 · base64 · serde · bs58
```

> **Note:** The app works without Rust — it falls back to pure-Python serialization — but it's definitely zippier with Rust!

---

## ⚡ Quick Start

```bash
# Clone the repository
git clone https://github.com/SuperKunten/TimeLocky.git
cd TimeLocky

# Install Python dependencies
pip install -r requirements.txt

# (Optional) Build the Rust extension
maturin develop --release

# Run
python main.py
```

---

## 📬 Links & Contact

| | |
|---|---|
| 💬 **Telegram Channel** | [t.me/timelocky](https://t.me/timelocky) |
| 👤 **Developer** | @bandokaay_ofb |
| 🐙 **GitHub** | [SuperKunten/TimeLocky](https://github.com/SuperKunten/TimeLocky) |

---

<p align="center">
  Made with ❤️ on Solana
</p>
