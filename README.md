
###  🏦 Program Derived Address (PDA) Wallet on Solana

A production-ready **Solana Smart Contract** for secure, decentralized wallet management built with the **Anchor framework**. This program utilizes Program Derived Addresses (PDAs) for account ownership and handles native SOL and SPL Token transfers with built-in security and transaction tracking.

![Solana](https://img.shields.io/badge/Solana-1.17%2B-blue?logo=solana)
![Anchor](https://img.shields.io/badge/Anchor-0.29.0-purple)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-green)

## ✨ Core Features & Value Proposition

| Feature | Description | Benefit |
| :--- | :--- | :--- |
| **🔐 PDA Account Model** | Wallet accounts are secured via Program Derived Addresses, owned by the program. | **Enhanced Security**; eliminates private key management for wallet accounts. |
| **💸 SOL & SPL Token Support** | Integrated instructions for native SOL transfers and SPL Token Cross-Program Invocations (CPIs). | **Full Utility**; handles all major asset classes on Solana. |
| **📊 Transaction Tracking** | Incremental counter stored on the wallet account for auditability. | **Auditability**; maintains a verifiable history of all activity. |
| **🛡️ Robust Security** | Comprehensive ownership validation, balance checks, and custom error handling. | **Production-Ready Safety**; minimizes exploit vectors and ensures transactional integrity. |

---

## 🚀 Getting Started (Quick Setup)

### Prerequisites

Ensure you have the following tools installed:

- **Rust** 1.70.0 or later
- **Solana CLI** 1.17.0 or later
- **Anchor CLI** 0.29.0 or later
- **Node.js** 18.0 or later

### Installation & Testing

```bash
# 1. Clone the repository
git clone https://github.com/YOUR_USERNAME/solana-wallet-program.git
cd solana-wallet-program

# 2. Install TypeScript/Node dependencies
npm install

# 3. Build the smart contract
anchor build

# 4. Run the comprehensive test suite
anchor test
```

## 🏗 Program Architecture

This program follows the recommended modular structure for Anchor projects, separating logic, state, and errors for clarity and maintainability.

### Project Structure Overview

```
solana-wallet-program/
├── programs/
│   └── wallet-program/
│       ├── src/
│       │   ├── lib.rs                 # Program entry, event emission
│       │   ├── instructions/          # Instruction handlers (initialize, transfer)
│       │   ├── state/                 # Account structures (WalletAccount)
│       │   └── errors.rs              # Custom error codes
├── tests/
│   └── wallet-program.ts              # Comprehensive test suite (TypeScript)
├── migrations/
│   └── deploy.ts                      # Deployment script
└── Anchor.toml                        # Anchor configuration
```

## 🔬 Testing & Verification

The included test suite provides high-fidelity coverage for all critical program paths, ensuring reliability and security.

### Test Commands

```bash
anchor test
```

### Key Test Coverage Areas

- ✅ **Wallet Initialization**: Successful creation and storage of the WalletAccount PDA.
- ✅ **SOL Transfers**: Correct execution of SOL transfers with strict balance validation checks.
- ✅ **SPL Token Transfers**: Successful Cross-Program Invocations (CPIs) to the Token Program.
- ✅ **Access Control**: Verification of Unauthorized and security error handling.
- ✅ **State Tracking**: Correct incrementing of the transaction_count.

## 🚢 Deployment

### Local Validator Deployment

Use the local validator for rapid development and iteration.

```bash
# 1. Start local validator in a separate terminal
solana-test-validator

# 2. Deploy the program locally
anchor build
anchor deploy
```

### Devnet Deployment

To deploy and test on the public Devnet cluster:

```bash
# Set cluster to devnet
solana config set --url devnet

# Deploy
anchor deploy --provider.cluster devnet

# Tip: Request Devnet SOL for testing
solana airdrop 2
```

## 📘 API Reference

### Account State

The central account structure for the wallet.

#### `WalletAccount` (State)

```rust
pub struct WalletAccount {
    pub owner: Pubkey,              // The public key of the user who owns this wallet.
    pub created_at: i64,            // Unix timestamp of creation.
    pub transaction_count: u64,     // Total number of successful transactions.
    pub bump: u8,                   // The PDA bump seed used for derivation.
}
```

### Instructions

#### `initialize_wallet`

Initializes a new PDA wallet account.

- **Signer**: `owner`
- **PDA**: `wallet` (Derives from owner key and a unique seed)

#### `transfer_sol`

Transfers a specified amount of native SOL from the wallet to a recipient.

- **Signer**: `sender` (Must be the owner of the wallet)
- **Parameter**: `amount: u64` (in Lamports)

#### `transfer_spl_token`

Transfers a specified amount of SPL tokens between two token accounts. Requires the wallet's PDA to sign the CPI.

- **Signer**: `authority` (Must be the owner of the wallet)
- **Parameter**: `amount: u64` (in token base units)

### Custom Errors

All program errors are explicitly defined in `errors.rs` for clear debugging and client-side handling.

| Error Code | Description |
| :--- | :--- |
| `InsufficientBalance` | The wallet has less SOL/tokens than requested. |
| `Unauthorized` | The caller is not the owner of the wallet. |
| `InvalidRecipient` | The recipient address failed validation checks. |

## 💡 Skills Demonstrated (For Developers)

This project is a showcase of advanced Solana development patterns:

- **Anchor Framework Mastery**: Demonstrating proficient use of Anchor's macros, constraints, and testing utilities.
- **Program Derived Addresses (PDAs)**: Securely deriving and managing accounts via PDAs for non-custodial ownership.
- **Cross-Program Invocations (CPIs)**: Safe interaction with the SPL Token Program and System Program.
- **Smart Contract Security**: Implementing strict access control, validation checks, and reentrancy guards (via Anchor defaults).
- **Rust Development**: Clean, idiomatic, and performant Rust code utilizing common crates (`anchor-lang`, `spl-token`).

## 🤝 Contribution

We welcome contributions! Please feel free to submit issues and pull requests to help improve this project.

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---
**⭐ If you find this repository useful, please consider starring it!**
