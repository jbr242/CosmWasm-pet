import { SecretNetworkClient, Wallet } from "secretjs";
import * as dotenv from "dotenv";
import * as readline from "readline";

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

const instantiateContract = async (codeId: string, contractCodeHash: string, petName): Promise<string> => {
    // The instantiate message is empty in this example. 
    // We could also send an `admin` address if we wanted to.
    
    const initMsg = {
        name: petName,  // Replace with your pet's name
        // owner: wallet.address,  // Optional; defaults to the sender
    };
    let tx = await secretjs.tx.compute.instantiateContract(
        {
            code_id: codeId,
            sender: wallet.address,
            code_hash: contractCodeHash,
            init_msg: initMsg,
            label: "Pet Contract" + Math.ceil(Math.random() * 10000000),
        },
        {
            gasLimit: 400_000,
        }
    );
    //Find the contract_address in the logs
    //@ts-ignore
    const contractAddress = tx.arrayLog!.find((log) => log.type === "message" && log.key === "contract_address").value;
    
    return contractAddress;
};

export const main = async (): Promise<void> => {

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
    let code_id = ""
    let code_hash = ""

    const fromEnv = await question("Do you want to use the code id and code hash from the .env file? (y/n): ");
    if (fromEnv === "y") {
        try {
            if (!process.env.CODE_ID || !process.env.CODE_HASH) {
                throw new Error("CODE_ID or CODE_HASH not found in .env file");
            }
        }
        catch (err) {
            console.error(err);
            process.exit(1);
        }
        code_id = process.env.CODE_ID;
        code_hash = process.env.CODE_HASH;
        
    } else {
        // Prompt the user for the pet name and password
        code_id = await question("Enter the code id: ");
        code_hash = await question("Enter the code hash: ");
    }

    const petName = await question("Enter your pet's name: ");
    const password = await question("Enter a password for your pet: ");

    // Close the readline interface
    rl.close();

    const contract_address = await instantiateContract(code_id, code_hash, petName);

    const set_password_msg = {
        set_password: {
            password: password,
        },
    };

    const set_password_tx = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address,
            code_hash,
            msg: set_password_msg,
            sent_funds: [], // Optional
        },
        {
            gasLimit: 100_000,
        },
    );
    if (set_password_tx.code !== 0) {
        throw new Error(`Failed to set password: ${set_password_tx.rawLog}`);
    } else {
        console.log("Set password successfully, cost:", set_password_tx.gasUsed);
    }
    console.log("This is your pets contract address: ", contract_address);
}

main()