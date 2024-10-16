import { SecretNetworkClient, Wallet } from "secretjs";
import * as dotenv from "dotenv";

dotenv.config();  // Load environment variables from .env file 
const mnemonic = process.env.MNEMONIC;  // Retrieve the mnemonic

const wallet = new Wallet(mnemonic);

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

interface GetStatus {
  get_status: {
    name: string;
    hunger_level: number;
    happiness_level: number;
    energy_level: number;
  }
}

const instantiateContract = async (codeId: string, contractCodeHash: string, petName: string): Promise<string> => {
    const initMsg = {
        name: petName,
    };
    const tx = await secretjs.tx.compute.instantiateContract(
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
    //@ts-ignore
    const contractAddress = tx.arrayLog!.find((log) => log.type === "message" && log.key === "contract_address").value;
    return contractAddress;
};

const main = async () => {
    // Get code_id and code_hash from environment variables
    const code_id = process.env.CODE_ID;
    const code_hash = process.env.CODE_HASH;

    if (!code_id || !code_hash) {
        console.error("CODE_ID or CODE_HASH not found in .env file");
        process.exit(1);
    }

    const petName = "TestPet";
    const password = "TestPassword";

    console.log("Instantiating contract...");

    const contract_address = await instantiateContract(code_id, code_hash, petName);

    console.log("Contract instantiated at address:", contract_address);

    // Set password
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
            sent_funds: [],
        },
        {
            gasLimit: 100_000,
        },
    );
    if (set_password_tx.code !== 0) {
        throw new Error(`Failed to set password: ${set_password_tx.rawLog}`);
    } else {
        console.log("Set password successfully, gas used:", set_password_tx.gasUsed);
    }

    // Get initial pet status
    const get_status_query = {
        get_status: {
            password,
        },
    };

    let get_status_result = await secretjs.query.compute.queryContract({
        contract_address,
        code_hash,
        query: get_status_query,
    }) as GetStatus;

    console.log("Initial pet status:", get_status_result.get_status);

    // Store initial levels
    let initial_hunger = get_status_result.get_status.hunger_level;
    let initial_happiness = get_status_result.get_status.happiness_level;
    let initial_energy = get_status_result.get_status.energy_level;

    // Feed the pet
    const feedAmount = 5;

    const feed_msg = {
        feed: {
            amount: feedAmount,
        },
    };

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
    if (feed_tx.code !== 0) {
        throw new Error(`Failed to feed the pet: ${feed_tx.rawLog}`);
    } else {
        console.log(`Fed the pet ${feedAmount} times, gas used: ${feed_tx.gasUsed}`);
    }

    // Get pet status after feeding
    get_status_result = await secretjs.query.compute.queryContract({
        contract_address,
        code_hash,
        query: get_status_query,
    }) as GetStatus;

    console.log("Pet status after feeding:", get_status_result.get_status);

    // Check that hunger_level decreased by feedAmount
    const hunger_after_feeding = get_status_result.get_status.hunger_level;
    const expected_hunger = initial_hunger - feedAmount;

    if (hunger_after_feeding !== expected_hunger) {
        console.error(`Error: Hunger level after feeding (${hunger_after_feeding}) does not match expected (${expected_hunger})`);
    } else {
        console.log("Hunger level after feeding is correct.");
    }

    // Play with the pet
    const playAmount = 3;

    const play_msg = {
        play: {
            amount: playAmount,
        },
    };

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
    if (play_tx.code !== 0) {
        throw new Error(`Failed to play with the pet: ${play_tx.rawLog}`);
    } else {
        console.log(`Played with the pet ${playAmount} times, gas used: ${play_tx.gasUsed}`);
    }

    // Get pet status after playing
    get_status_result = await secretjs.query.compute.queryContract({
        contract_address,
        code_hash,
        query: get_status_query,
    }) as GetStatus;

    console.log("Pet status after playing:", get_status_result.get_status);

    // Check that happiness_level increased by playAmount
    const happiness_after_playing = get_status_result.get_status.happiness_level;
    const expected_happiness = initial_happiness + playAmount;

    if (happiness_after_playing !== expected_happiness) {
        console.error(`Error: Happiness level after playing (${happiness_after_playing}) does not match expected (${expected_happiness})`);
    } else {
        console.log("Happiness level after playing is correct.");
        initial_energy -= 1; // Energy decreases by 1 when played with
    }

    // Rest the pet
    const restAmount = 4;

    const rest_msg = {
        rest: {
            amount: restAmount,
        },
    };

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
    if (rest_tx.code !== 0) {
        throw new Error(`Failed to rest the pet: ${rest_tx.rawLog}`);
    } else {
        console.log(`Rested the pet ${restAmount} times, gas used: ${rest_tx.gasUsed}`);
    }

    // Get pet status after resting
    get_status_result = await secretjs.query.compute.queryContract({
        contract_address,
        code_hash,
        query: get_status_query,
    }) as GetStatus;

    console.log("Pet status after resting:", get_status_result.get_status);

    // Check that energy_level increased by restAmount
    const energy_after_resting = get_status_result.get_status.energy_level;
    const expected_energy = initial_energy + restAmount; 

    if (energy_after_resting !== expected_energy) {
        console.error(`Error: Energy level after resting (${energy_after_resting}) does not match expected (${expected_energy})`);
    } else {
        console.log("Energy level after resting is correct.");
    }

    // Check if the pet is hungry
    const is_hungry_query = {
        is_hungry: {
            password,
        },
    };

    const isHungryResult = await secretjs.query.compute.queryContract({
        contract_address,
        code_hash,
        query: is_hungry_query,
    }) as { is_hungry: { is_hungry: boolean } };

    if (isHungryResult.is_hungry.is_hungry) {
        console.log("The pet is hungry.");
    } else {
        console.log("The pet is not hungry.");
    }

    console.log("All tests completed successfully.");
};

main();
