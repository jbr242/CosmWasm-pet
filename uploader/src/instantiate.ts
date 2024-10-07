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

const instantiateContract = async (codeId: string, contractCodeHash: string): Promise<string> => {
    // The instantiate message is empty in this example. 
    // We could also send an `admin` address if we wanted to.
    
    const initMsg = {};
    // const initMsg = {
    //     admin: wallet.address
    // };
    let tx = await secretjs.tx.compute.instantiateContract(
        {
            code_id: codeId,
            sender: wallet.address,
            code_hash: contractCodeHash,
            init_msg: initMsg,
            label: "test contract" + Math.ceil(Math.random() * 10000000),
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

    const contract_address = await instantiateContract(code_id, code_hash);
    
    console.log("Contract address: ", contract_address);

    // query `auction_info`
    let auction_info_result = await secretjs.query.compute.queryContract({
        contract_address,
        code_hash,
        query: {
            auction_info: { }
        }
    });
    console.log(auction_info_result);

    // Date.now gives a time in milliseconds, convert to seconds 
    // and add 120 seconds 
    let end_time = (Math.floor(Date.now() / 1000) + 120).toString(); 
    // create the `set_auction` message
    let set_auction_msg = {
        set_auction: {
            secret: "This is my secret!",
            minimum_bid: "1000000", // 1,000,000 uscrt == 1 SCRT
            end_time
        }
    };
    console.log(set_auction_msg);
    // execute `set_auction`
    const set_auction_tx = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address,
            code_hash,
            msg: set_auction_msg,
            sent_funds: [], // optional
        },
        {
            gasLimit: 100_000,
        },
    );
    console.log(set_auction_tx);

    // execute `start_auction`
    const start_auction_tx = await secretjs.tx.compute.executeContract(
        {
            sender: wallet.address,
            contract_address,
            code_hash,
            msg: { start_auction: { } },
            sent_funds: [], // optional
        },
        {
            gasLimit: 100_000,
        },
    );
    console.log(start_auction_tx);

    // query `auction_info` again. It now says started
    auction_info_result = await secretjs.query.compute.queryContract({
        contract_address,
        code_hash,
        query: {
            auction_info: { }
        }
    });
    console.log(auction_info_result);
}

main()