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
│       │   ├── lib.rs
│       │   ├── staking/
│       │   │   ├── mod.rs
│       │   │   ├── initialize.rs
│       │   │   ├── deposit.rs
│       │   │   ├── withdraw.rs
│       │   │   └── state.rs
│       └── Cargo.toml
├── tests/
│   └── integration/
│       └── staking.test.ts
├── Anchor.toml
├── package.json
├── tsconfig.json
└── README.md
Setup

Clone the Repository
Replace <your-repo-url> with your GitHub repository URL:
bashgit clone <your-repo-url>
cd solana-staking-contract/staking

Initialize Git Repository
If starting fresh, initialize Git:
bashgit init
git add .
git commit -m "Initial commit for Solana staking contract"
git remote add origin <your-repo-url>
git push -u origin main

Install Dependencies
Install Yarn dependencies for testing:
bashyarn install

Set Up Solana Localnet
Configure Solana CLI to use localnet:
bashsolana config set --url http://localhost:8899
Start the local Solana validator in a separate terminal:
bashsolana-test-validator

Verify Wallet
Ensure your Solana wallet has sufficient SOL for deployment and testing:
bashsolana balance
solana airdrop 5  # Airdrop 5 SOL if needed


Building the Program
Compile the Rust program using Anchor:
bashanchor build
Alternatively, use Cargo to build for Solana’s BPF target:
bashcargo build-bpf
The compiled program will be in target/deploy/staking.so.
If build errors occur, clean artifacts and retry:
bashanchor clean
anchor build
Capture build output for debugging:
bashanchor build > build.log 2>&1
cat build.log
Running the Program
To run the program manually (e.g., for debugging):
bashcargo run --bin staking
Note: This command builds and runs the program but does not interact with it. For testing and interaction, use anchor test or a custom client script. Ensure the program is built (anchor build) and deployed (anchor deploy) before running.
Deploying Locally
Deploy the program to the local Solana validator:
bashanchor deploy
Ensure solana-test-validator is running. Verify deployment:
bashsolana program show Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
The program ID is Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS, defined in lib.rs and Anchor.toml.
If deployment fails, check wallet balance and validator status:
bashsolana balance
solana airdrop 5
Running Tests
Run integration tests to verify the contract’s functionality:
bashanchor test
Ensure solana-test-validator is running. The tests:

Create a mock SPL token.
Initialize the vault with a token mint and vault token account.
Deposit tokens and verify the staking account balance.
Withdraw tokens and verify the updated balance.
Attempt to withdraw more than the staked balance (should fail with "Insufficient balance").

Capture test output for debugging:
bashanchor test > test.log 2>&1
cat test.log
Program Details

Program ID: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
Accounts:

Vault: Stores the token mint address (space: 8 + 32 bytes).
StakingAccount: Tracks each user’s staked balance and owner (space: 8 + 32 + 8 bytes).


Instructions:

initialize: Creates the vault and initializes a token account for the vault.
deposit: Transfers tokens from the user to the vault and updates the staking account balance.
withdraw: Transfers tokens from the vault to the user, checking for sufficient balance and ownership.


Error Handling:

Overflow: Prevents arithmetic overflow in deposits.
Underflow: Prevents withdrawing more than the staked balance.
Unauthorized: Ensures only the staking account owner can withdraw.


Safety: The init_if_needed in deposit.rs is safe due to unique seeds (signer.key() and vault.key()), preventing re-initialization attacks.