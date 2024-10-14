import { SecretNetworkClient, Wallet } from "secretjs";
import * as dotenv from "dotenv";
import * as readline from "readline";

dotenv.config();  // Load environment variables from .env file
const mnemonic = process.env.MNEMONIC!;  // Retrieve the mnemonic

const wallet = new Wallet(mnemonic);

// Create a new client for the Pulsar testnet
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

export const main = async (): Promise<void> => {
  // Create a readline interface for user input
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  // Promisify the question method to use async/await
  const question = (questionText: string): Promise<string> => {
    return new Promise((resolve) => {
      rl.question(questionText, (answer) => {
        resolve(answer);
      });
    });
  };

  // Prompt the user for the contract address and code hash
  const contract_address = await question("Enter the contract address: ");
  const code_hash = await question("Enter the contract code hash: ");
  const password = await question("Enter your pet's password: ");

  console.log("\nWelcome to your Pet\n");

  // Main interaction loop
  let exit = false;
  while (!exit) {
    console.log("Please choose an action:");
    console.log("1. Feed the pet");
    console.log("2. Play with the pet");
    console.log("3. Rest the pet");
    console.log("4. Get pet status");
    console.log("5. Check if the pet is hungry");
    console.log("6. Transfer ownership");
    console.log("7. Exit");

    const choice = await question("Enter the number of your choice: ");

    switch (choice.trim()) {
      case "1":
        // Feed the pet
        const feedAmountStr = await question("Enter the amount to feed (1-10): ");
        const feedAmount = parseInt(feedAmountStr, 10);

        if (isNaN(feedAmount) || feedAmount < 1 || feedAmount > 10) {
          console.log("Invalid amount. Please enter a number between 1 and 10.\n");
          break;
        }

        const feed_msg = {
          feed: {
            amount: feedAmount,
          },
        };

        try {
          const feed_tx = await secretjs.tx.compute.executeContract(
            {
              sender: wallet.address,
              contract_address,
              code_hash,
              msg: feed_msg,
              sent_funds: [],
            },
            {
              gasLimit: 100_000,
            },
          );
          console.log("Feed transaction successful:", feed_tx.transactionHash, "\n");
        } catch (error) {
          console.error("Error feeding the pet:", error, "\n");
        }
        break;

      case "2":
        // Play with the pet
        const playAmountStr = await question("Enter the amount to play (1-10): ");
        const playAmount = parseInt(playAmountStr, 10);

        if (isNaN(playAmount) || playAmount < 1 || playAmount > 10) {
          console.log("Invalid amount. Please enter a number between 1 and 10.\n");
          break;
        }

        const play_msg = {
          play: {
            amount: playAmount,
          },
        };

        try {
          const play_tx = await secretjs.tx.compute.executeContract(
            {
              sender: wallet.address,
              contract_address,
              code_hash,
              msg: play_msg,
              sent_funds: [],
            },
            {
              gasLimit: 100_000,
            },
          );
          console.log("Play transaction successful:", play_tx.transactionHash, "\n");
        } catch (error) {
          console.error("Error playing with the pet:", error, "\n");
        }
        break;

      case "3":
        // Rest the pet
        const restAmountStr = await question("Enter the amount to rest (1-10): ");
        const restAmount = parseInt(restAmountStr, 10);

        if (isNaN(restAmount) || restAmount < 1 || restAmount > 10) {
          console.log("Invalid amount. Please enter a number between 1 and 10.\n");
          break;
        }

        const rest_msg = {
          rest: {
            amount: restAmount,
          },
        };

        try {
          const rest_tx = await secretjs.tx.compute.executeContract(
            {
              sender: wallet.address,
              contract_address,
              code_hash,
              msg: rest_msg,
              sent_funds: [],
            },
            {
              gasLimit: 100_000,
            },
          );
          console.log("Rest transaction successful:", rest_tx.transactionHash, "\n");
        } catch (error) {
          console.error("Error resting the pet:", error, "\n");
        }
        break;

      case "4":
        // Get pet status
        const get_status_query = {
          get_status: {
            password,
          },
        };

        try {
          const get_status_result = await secretjs.query.compute.queryContract({
            contract_address,
            code_hash,
            query: get_status_query,
          });

          console.log("Pet status:", get_status_result, "\n");
        } catch (error) {
          console.error("Error getting pet status:", error, "\n");
        }
        break;

      case "5":
        // Check if the pet is hungry
        const is_hungry_query = {
          is_hungry: {
            password,
          },
        };

        try {
          const is_hungry_result = await secretjs.query.compute.queryContract({
            contract_address,
            code_hash,
            query: is_hungry_query,
          });

          console.log("Is the pet hungry?", is_hungry_result, "\n");
        } catch (error) {
          console.error("Error checking if pet is hungry:", error, "\n");
        }
        break;

      case "6":
        // Transfer ownership
        const newOwner = await question("Enter the new owner's address: ");

        const transfer_ownership_msg = {
          transfer: {
            new_owner: newOwner,
          },
        };

        try {
          const transfer_ownership_tx = await secretjs.tx.compute.executeContract(
            {
              sender: wallet.address,
              contract_address,
              code_hash,
              msg: transfer_ownership_msg,
              sent_funds: [],
            },
            {
              gasLimit: 100_000,
            },
          );
          console.log("Transfer ownership transaction successful:", transfer_ownership_tx.transactionHash, "\n");
        } catch (error) {
          console.error("Error transferring ownership:", error, "\n");
        }
        break;
      case "7":
        // Exit the script
        console.log("Exiting the script. Goodbye!");
        exit = true;
        break;

      default:
        console.log("Invalid choice. Please enter a number between 1 and 6.\n");
        break;
    }
  }
  rl.close();
};

main();
