# README

## Pet Contract Interaction Scripts

This project contains scripts to interact with a pet contract deployed on the Secret Network's Pulsar testnet. The scripts allow you to:

- **Upload** the pet contract to the network.
- **Instantiate** a new pet contract instance.
- **Execute actions** like feeding, playing, and resting with your pet.
- **Query** your pet's status.
- **Transfer ownership** of your pet.
- **Run automated tests** to verify contract functionality.

---

## Table of Contents

- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Setup Instructions](#setup-instructions)
- [Environment Variables](#environment-variables)
- [Scripts Overview](#scripts-overview)
- [Running the Scripts](#running-the-scripts)
  - [1. Build the Contract](#1-build-the-contract)
  - [2. Upload the Contract](#2-upload-the-contract)
  - [3. Instantiate the Contract](#3-instantiate-the-contract)
  - [4. Interact with Your Pet](#4-interact-with-your-pet)
  - [5. Run Tests](#5-run-tests)

---

## Project Structure

The repository is organized as follows:

- **Root Directory**: Contains the contract files (e.g., Rust source code for the smart contract).
- **`uploader/`**: A folder inside the main repository that contains all the scripts for interacting with the contract.
  - **Scripts**: `upload.ts`, `instantiate.ts`, `execute.ts`, `test.ts`, etc.

---

## Prerequisites

- **Node.js** (version 14 or higher)
- **npm** 
- **TypeScript** (scripts use `ts-node` for execution)
- **Rust and Cargo**: For building the smart contract
- An account on the **Secret Network Pulsar Testnet**
- **Testnet SCRT tokens** for transaction fees

---

## Setup Instructions

1. **Clone the Repository**

   ```bash
   git clone [repo]
   cd cosmwasm-pet/uploader
   ```

   - The `uploader` directory contains all the scripts you'll use.

2. **Install Dependencies**

   ```bash
   npm install
   ```

3. **Create a `.env` File**

   Create a `.env` file in the `uploader` directory to store your environment variables.


   **Important:** Do not commit this file to version control.

4. **Set Up Environment Variables**

   Open the `.env` file and add the following variables:

   ```dotenv
   MNEMONIC="your wallet mnemonic"
   CODE_ID="your contract's code ID (will be set after upload)"
   CODE_HASH="your contract's code hash (will be set after upload)"
   PET_ADDRESS="your pet's contract address (will be set after instantiation)"
   ```

   - **MNEMONIC**: Your wallet's 24-word mnemonic seed phrase.
   - **CODE_ID**: The code ID of the pet contract (set after uploading the contract).
   - **CODE_HASH**: The code hash of the pet contract (set after uploading the contract).
   - **PET_ADDRESS**: The contract address of your pet (set after instantiation).

   **Example:**

   ```dotenv
   MNEMONIC="gesture rather obey video brave void wish flame noodle exist middle gym kitten rifle novel dumb mad pet viable outer duck hero emerge talent"
   ```

---

## Environment Variables

- **MNEMONIC**: *(Required)* Your wallet's mnemonic phrase for signing transactions.
- **CODE_ID**: *(Set after upload)* The code ID returned when you upload your contract to the network.
- **CODE_HASH**: *(Set after upload)* The code hash associated with your contract.
- **PET_ADDRESS**: *(Set after instantiation)* The contract address of your pet.

---

## Scripts Overview

All scripts are located inside the `uploader/` directory.

- **Upload Script**: `upload.ts`
- **Instantiate Script**: `instantiate.ts`
- **Execute Script**: `execute.ts`
- **Test Script**: `test.ts`

---

## Running the Scripts

### 1. Build the Contract

Before uploading the contract, you need to compile it. From the root directory (where your contract files are located), run:
```bash
cd ..
```

```bash
make build-mainnet-reproducible
```

This command compiles your smart contract and produces a compressed Wasm file (`contract.wasm.gz`) suitable for uploading to the Secret Network.

### 2. Upload the Contract

After building the contract, navigate to the `uploader/` directory:

```bash
cd uploader
```

Upload the compiled Wasm file to the Secret Network:

```bash
npm run upload
```

**Steps:**

- The script will upload the contract and output the `code_id` and `code_hash`:

  ```
  codeId: 1234
  Contract hash: abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
  Code ID: 1234
  Code hash: abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890
  ```

- **Important:** Add the `CODE_ID` and `CODE_HASH` values to your `.env` file.

  ```dotenv
  CODE_ID="1234"
  CODE_HASH="abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
  ```

### 3. Instantiate the Contract

After uploading, you need to instantiate the contract to create a pet instance.

```bash
npm run instantiate
```

**Steps:**

- The script will prompt:

  ```
  Do you want to use the code id and code hash from the .env file? (y/n):
  ```

  - Enter `y` to use the values from your `.env` file.
  - Enter `n` to input them manually.

- Next, you will be asked:

  ```
  Enter your pet's name:
  ```

  - Provide a name for your pet.

  ```
  Enter a password for your pet:
  ```

  - Set a password to secure your pet.

- Upon successful instantiation, the script will display your pet's contract address:

  ```
  This is your pet's contract address: secret1...
  ```

- **Important:** Add this contract address to your `.env` file as `PET_ADDRESS` for convenience.

  ```dotenv
  PET_ADDRESS="secret1abcdefghijklmopqrstuvwxyz1234567"
  ```

### 4. Interact with Your Pet

Now you can interact with your pet using the execute script.

```bash
npm run execute
```

**Steps:**

- The script will prompt:

  ```
  Do you want to use the code hash and pet address from the .env file? (y/n):
  ```

  - Enter `y` to use the values from your `.env` file.
  - Enter `n` to input them manually.

- Then, you'll be asked for your pet's password:

  ```
  Enter your pet's password:
  ```

- You'll see a welcome message and a menu:

  ```
  Welcome to your Pet

  Please choose an action:
  1. Feed the pet
  2. Play with the pet
  3. Rest the pet
  4. Get pet status
  5. Check if the pet is hungry
  6. Transfer ownership
  7. Exit
  ```

- **Select an option** by entering the corresponding number.

- **Follow the prompts** to perform actions with your pet.

### 5. Run Tests

To verify that the contract is functioning correctly, run the test script.

```bash
npm run test
```

**What the Test Script Does:**

- Instantiates a new pet contract with test parameters.
- Sets a password for the pet.
- Performs actions: feed, play, rest.
- Queries the pet's status after each action.
- Checks if the pet's levels (hunger, happiness, energy) have changed as expected.
- Outputs the results and any discrepancies found.

---
