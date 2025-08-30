# Solana Staking Smart Contract

This is a simple staking smart contract built on Solana using Rust and Anchor. It allows users to deposit a mock SPL token, tracks their staked balance, and enables withdrawals at any time.

## Prerequisites

- **Rust**: Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
- **Solana CLI**: Install with `sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"`.
- **Anchor CLI**: Install with `cargo install --git https://github.com/coral-xyz/anchor avm --locked --force` and `avm install latest`.
- **Node.js**: Install Node.js and Yarn (`npm install -g yarn`).
- **Git**: For version control.

## Setup

1. **Clone the Repository**
   ```bash
   git clone <your-repo-url>
   cd solana-staking-contract/staking