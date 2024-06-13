import * as idl from "../target/idl/lockbox_governor.json";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createMint, mintTo, transfer, getOrCreateAssociatedTokenAccount, syncNative, createAssociatedTokenAccount,
  unpackAccount, TOKEN_PROGRAM_ID, AccountLayout, getAssociatedTokenAddress, setAuthority, AuthorityType
} from "@solana/spl-token";
import expect from "expect";
import fs from "fs";

// NOTE!!!! Make sure you run lockbox_governor_init.ts script first to initialize the governor program
// NOTE!!!! Run this script only after all the following steps are executed strictly in order:
// 1. Deploy governor program by running the lockbox_governor_init.ts script (README)
// 2. Deploy liquidity_lockbox (1okwt4nGbpr82kkr6t1767sAenfeZBxUyzJAAaumZRG) from artifacts or use another key-pair
//    Make sure you are the program authority. Example:
//      solana program deploy --program-id artifacts/1okwt4nGbpr82kkr6t1767sAenfeZBxUyzJAAaumZRG.json artifacts/liquidity_lockbox.so --url localhost
// 3. Change the deployed program authority to pdaConfig (CuZVidD5KhTGN2jc931uH4EBAErzYWCUiLJUVA9NtLHw). Example:
//    solana program set-upgrade-authority 1okwt4nGbpr82kkr6t1767sAenfeZBxUyzJAAaumZRG --new-upgrade-authority CuZVidD5KhTGN2jc931uH4EBAErzYWCUiLJUVA9NtLHw --skip-new-upgrade-authority-signer-check --url localhost
// 4. Prepare a re-deployment of another lockbox program version via the buffer.
//    Deploy the program into the buffer. Example:
//      solana program write-buffer artifacts/liquidity_lockbox.so --url localhost
//    Change the buffer authority to pdaConfig (CuZVidD5KhTGN2jc931uH4EBAErzYWCUiLJUVA9NtLHw). Example:
//      solana program set-buffer-authority EpFxEcCuf1mndxvkYsoYhuXb8oYWvDsGwYCq5gLVqzv3 --new-buffer-authority CuZVidD5KhTGN2jc931uH4EBAErzYWCUiLJUVA9NtLHw --url localhost
//    Check authority of the program and of the buffer, and they must match:
//      solana program show 1okwt4nGbpr82kkr6t1767sAenfeZBxUyzJAAaumZRG --url localhost
//
// Note: Step 4 is not needed for testing. The buffer address is different every time the program is written into the buffer,
//       so EpFxEcCuf1mndxvkYsoYhuXb8oYWvDsGwYCq5gLVqzv3 is a one-time example. However, it is dumped as an account,
//       and added in the validator initialization. Use this account to fully replicate the test by executing steps 1-3.
// Reference: https://docs.solanalabs.com/ru/cli/examples/deploy-a-program

// UNIX/Linux/Mac
// bash$ export ANCHOR_PROVIDER_URL=http://127.0.0.1:8899
// bash$ export ANCHOR_WALLET=artifacts/id.json

async function main() {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const PROGRAM_ID = new anchor.web3.PublicKey("DWDGo2UkBUFZ3VitBfWRBMvRnHr7E2DSh57NK27xMYaB");
  const program = new Program(idl as anchor.Idl, PROGRAM_ID, anchor.getProvider());

  const chainId = 10002;
  const sequence = 13;

  // Deploy this manually with the solana program deploy...
  const lockbox = new anchor.web3.PublicKey("1okwt4nGbpr82kkr6t1767sAenfeZBxUyzJAAaumZRG");
  const lockboxData = new anchor.web3.PublicKey("Gdt3RDEQmw51NCcUJ13tXR6nj9sgKMaZe1Pic8JSRDfb");
  const lockbox2Buffer = new anchor.web3.PublicKey("EpFxEcCuf1mndxvkYsoYhuXb8oYWvDsGwYCq5gLVqzv3");
  const wormhole = new anchor.web3.PublicKey("3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5");
  const posted = new anchor.web3.PublicKey("Dy9s52xnX2yNsSE9Z2c24eRMfTLEzd46iJn7hYkk83nW");
  const bpfLoaderUpgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
  const sol = new anchor.web3.PublicKey("So11111111111111111111111111111111111111112");

  // This corresponds to Sepolia timelock address 000000000000000000000000471b3f60f08c50dd0ecba1bcd113b66fcc02b63d or 0x471b3f60f08c50dd0ecba1bcd113b66fcc02b63d
  const timelockBuffer = Buffer.from([
      0,   0,  0,   0,   0,   0,   0,   0,   0,
      0,   0,  0,  71,  27,  63,  96, 240, 140,
     80, 221, 14, 203, 161, 188, 209,  19, 182,
    111, 204,  2, 182,  61
  ]);
  const timelock = new anchor.web3.PublicKey(timelockBuffer);

  const vaaHashSetUpgradeAuthority = Buffer.from([
     13,  12, 162, 159, 168, 153, 181,
    243, 236, 253, 153, 127, 187, 110,
    220, 107,  59,  31,  97,  15, 180,
    122,  43, 168,  69, 208, 212, 198,
    196, 122, 169, 196
  ]);

  // User wallet is the provider payer
  const userWallet = provider.wallet["payer"];
  console.log("User wallet:", userWallet.publicKey.toBase58());

    // Find a PDA account for the lockbox governor program
    const [pdaConfig, bumpConfig] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("config", "utf-8")],
        program.programId);
    //let bumpBytes = Buffer.from(new Uint8Array([bumpConfig]));
    console.log("Lockbox Governor PDA address:", pdaConfig.toBase58());
    console.log("Lockbox Governor PDA bump:", bumpConfig);

    // Find a PDA account for the lockbox governor program
    let chainIdBuffer = Buffer.alloc(2);
    chainIdBuffer.writeUInt16LE(chainId, 0);
    let sequenceBuffer = Buffer.alloc(8);
    // NOTE! this needs to be adjusted with sequence number growing
    sequenceBuffer.writeUInt16LE(sequence, 0);
    const [pdaReceived, bumpReceived] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("received"),
        chainIdBuffer, sequenceBuffer], program.programId);
    //let bumpBytes = Buffer.from(new Uint8Array([bumpConfig]));
    console.log("Received PDA address:", pdaReceived.toBase58());
    console.log("Received PDA bump:", bumpReceived);

    let signature = null;
    // Set upgrade authority back
    try {
        signature = await program.methods
          .upgradeProgram(vaaHashSetUpgradeAuthority)
          .accounts(
            {
              config: pdaConfig,
              wormholeProgram: wormhole,
              posted,
              received: pdaReceived,
              programAccount: lockbox,
              programDataAccount: lockboxData,
              bufferAccount: lockbox2Buffer,
              spillAccount: userWallet.publicKey,
              bpfLoaderUpgradeable
            }
          )
          .rpc();
    } catch (error) {
        if (error instanceof Error && "message" in error) {
            console.error("Program Error:", error);
            console.error("Error Message:", error.message);
        } else {
            console.error("Transaction Error:", error);
        }
    }

    //console.log("Your transaction signature", signature);
    // Wait for program creation confirmation
    await provider.connection.confirmTransaction({
        signature: signature,
        ...(await provider.connection.getLatestBlockhash()),
    });

    console.log("Successfully re-deployed the program");
}

main();