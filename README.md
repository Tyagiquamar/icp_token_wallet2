ICP Token Wallet
Overview
This project is a secure, Rust-based token wallet built on the Internet Computer Protocol (ICP). The wallet supports fundamental functionalities such as sending and receiving tokens, viewing balances, and running unit tests. The project showcases my proficiency in Rust, smart contract development, and blockchain concepts.

This project was developed as part of an internship application to demonstrate my ability to build, test, and deploy decentralized applications on the ICP blockchain.

Features
Send Tokens: Transfer tokens to another user’s wallet using their Principal ID.
Receive Tokens: Receive tokens and automatically update the wallet balance.
View Balance: Retrieve and display the wallet's current token balance.
Unit Tests: Built-in tests to ensure the wallet functions correctly.
Project Structure

icp_token_wallet/
│
├── src/
│   ├── lib.rs                  # Main Rust code for the smart contract logic
│   ├── types.rs                # Structs and type definitions
│   ├── wallet.rs               # Core wallet logic (send, receive, get balance)
│   ├── tests.rs                # Unit tests for the wallet
│
├── dfx.json                    # DFX project configuration
├── Cargo.toml                  # Rust project configuration and dependencies
├── README.md                   # Project documentation
├── .gitignore                  # Files and folders to ignore in Git
├── .vscode/                    # VSCode configuration (optional)
│   ├── settings.json
│   ├── launch.json
│   ├── tasks.json
└── target/                     # Compiled WebAssembly (Wasm) output files
Prerequisites
Before setting up the project, ensure you have the following installed:

DFINITY SDK (DFX): Install DFX for managing the Internet Computer local environment.
Rust Toolchain: Install Rust and Cargo by following the Rust installation guide.
Node.js: Required if you plan to extend the project with a frontend (optional).
Installation and Setup
Follow these steps to set up and run the project locally:

1. Clone the Repository
Start by cloning the repository:

bash
git clone https://github.com/Perivo/icp_token_wallet.git
cd icp_token_wallet
2. Install Dependencies
Use Cargo to fetch and compile all necessary dependencies:

bash
cargo build
3. Start the Local ICP Network
Run a local replica of the Internet Computer to deploy and test your canisters:

bash
dfx start --background

4. Deploy the Smart Contract
Deploy the canister (smart contract) to the local ICP environment:

bash
dfx deploy
After a successful deployment, you will see the canister ID, which can be used to interact with the wallet.

Usage
Once deployed, you can interact with the wallet using the following commands:

Send Tokens:

bash
dfx canister call icp_token_wallet send_tokens "(\"recipient_principal\", 50)"
Replace recipient_principal with the Principal ID of the recipient and 50 with the amount of tokens you wish to send.

Receive Tokens:

bash
dfx canister call icp_token_wallet receive_tokens "(100)"
This command adds 100 tokens to your wallet balance.

Get Balance:

bash
dfx canister call icp_token_wallet get_balance
This retrieves and displays the current balance of your wallet.

Running Unit Tests
You can run the unit tests to validate the wallet’s core functionality:

bash
cargo test
The tests include scenarios for:

Receiving tokens.
Sending tokens with sufficient balance.
Handling errors when attempting to send tokens without enough balance.
Code Structure and Explanation
src/lib.rs
The entry point of the smart contract. It defines the public API for the canister, which includes functions like send_tokens, receive_tokens, and get_balance.

src/types.rs
Defines the core data structures and types used by the wallet, including structs for handling balances and transactions.

src/wallet.rs
Contains the core logic for the wallet, such as processing transactions, updating balances, and handling errors.

src/tests.rs
Includes comprehensive unit tests that validate the functionality of the wallet. The tests cover multiple scenarios to ensure the wallet operates as expected.
Troubleshooting
DFX Issues: Ensure the local ICP network is running with dfx start --background.
Cargo Build Errors: Verify that Rust is installed correctly and that dependencies are up to date.
