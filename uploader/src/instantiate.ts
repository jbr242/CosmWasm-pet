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
            label: "Pet Contract " + Math.ceil(Math.random() * 10000000),
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
    if (process.argv.length !== 4) {
        console.error('Expected two arguments!');
        process.exit(1);
    }

    let code_id = process.argv[2];
    let code_hash = process.argv[3];

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

    // Prompt the user for the pet name and password
    const petName = await question("Enter your pet's name: ");
    const userPassword = await question("Enter a password for your pet: ");

    // Close the readline interface
    rl.close();

    const contract_address = await instantiateContract(code_id, code_hash, petName);
    
    console.log("Contract address: ", contract_address);

    // Set the password for the pet contract
    const password = "my_secure_password";  // Replace with your desired password

    const set_password_msg = {
        set_password: {
            userPassword,
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
    console.log("Set password transaction:", set_password_tx);
}

main()