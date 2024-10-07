import { SecretNetworkClient, Wallet } from "secretjs";
import * as dotenv from "dotenv";

dotenv.config();  // Load environment variables from .env file 
const mnemonic = process.env.MNEMONIC;  // Retrieve the mnemonic

const wallet = new Wallet(mnemonic);

// create a new client for the Pulsar testnet
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar3.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

export const main = async (): Promise<void> => {
    const response = await secretjs.query.bank.balance(
        {
            address: secretjs.address,
            denom: "uscrt",
        }
    );
    console.log(response);
}

main()