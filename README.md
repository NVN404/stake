# Solana Staking Smart Contract

A simple, modular staking smart contract built on Solana using Rust and Anchor. Users can deposit SPL tokens, track their staked balance, and withdraw at any time. The contract is designed for clarity, security, and extensibility, with comprehensive integration tests.

---

## Features
- **Stake SPL tokens**: Users can deposit and withdraw tokens at will.
- **Per-user staking accounts**: Each user has a unique staking account.
- **Vault**: Program-controlled vault holds staked tokens securely.
- **Security**: Uses Anchor's `init_if_needed` with unique seeds to prevent re-initialization attacks.
- **Comprehensive tests**: Integration tests for all major flows and error cases.

---

## Prerequisites
- **Rust** (1.75.0+): [Install Rust](https://rustup.rs/)
- **Solana CLI** (1.18.0+): [Install Solana](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor CLI**: [Install Anchor](https://book.anchor-lang.com/chapter_1/installation.html)
- **Node.js** (16+), **Yarn**: `npm install -g yarn`
- **Git**

---

## Project Structure
```
staking/
├── programs/
│   └── staking/
│       ├── src/
│       │   ├── lib.rs
│       │   └── staking/
│       │       ├── mod.rs
│       │       ├── initialize.rs
│       │       ├── deposit.rs
│       │       ├── withdraw.rs
│       │       └── state.rs
│       └── Cargo.toml
├── tests/
│   └── staking.ts
├── Anchor.toml
├── package.json
├── tsconfig.json
└── README.md
```

---

## Setup

1. **Clone the repository**
  ```bash
  git clone https://github.com/NVN404/stake.git
  cd stake/staking
  ```
2. **Install dependencies**
  ```bash
  yarn install
  ```
3. **Configure Solana localnet**
  ```bash
  solana config set --url http://localhost:8899
  solana-test-validator
  ```
4. **Airdrop SOL to your wallet**
  ```bash
  solana airdrop 5
  solana balance
  ```

---

## Building & Deploying

1. **Build the program**
  ```bash
  anchor build
  # or
  cargo build-bpf
  ```
2. **Deploy locally**
  ```bash
  anchor deploy
  solana program show Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
  ```

---

## Testing

Run integration tests:
```bash
anchor test
```

Test coverage includes:
- Creating a mock SPL token
- Initializing the vault
- Depositing tokens
- Withdrawing tokens
- Failing to withdraw more than balance

---

## Program Details

- **Program ID**: `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS`

### Accounts
- **Vault**: Stores the token mint address
- **StakingAccount**: Tracks each user’s staked balance and owner

### Instructions
- `initialize`: Creates the vault and initializes a token account for the vault
- `deposit`: Transfers tokens from the user to the vault and updates the staking account balance
- `withdraw`: Transfers tokens from the vault to the user, checking for sufficient balance and ownership

### Error Handling
- **Overflow**: Prevents arithmetic overflow in deposits
- **Underflow**: Prevents withdrawing more than the staked balance
- **Unauthorized**: Ensures only the staking account owner can withdraw

---

## Security Notes
- The use of `init_if_needed` is safe due to unique seeds (user and vault keys), preventing re-initialization attacks. Review [Anchor docs](https://book.anchor-lang.com/chapter_6/faq.html#what-is-init_if_needed) for more details.

---

## License
MIT


