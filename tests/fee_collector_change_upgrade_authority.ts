import * as idl from "../target/idl/fee_collector.json";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createMint, mintTo, transfer, getOrCreateAssociatedTokenAccount, syncNative, createAssociatedTokenAccount,
  unpackAccount, TOKEN_PROGRAM_ID, AccountLayout, getAssociatedTokenAddress, setAuthority, AuthorityType
} from "@solana/spl-token";
import {
  WhirlpoolContext, buildWhirlpoolClient, ORCA_WHIRLPOOL_PROGRAM_ID,
  PDAUtil, PoolUtil, PriceMath, increaseLiquidityQuoteByInputTokenWithParams,
  decreaseLiquidityQuoteByLiquidityWithParams, TickUtil
} from "@orca-so/whirlpools-sdk";
import { DecimalUtil, Percentage } from "@orca-so/common-sdk";
import Decimal from "decimal.js";
import expect from "expect";
import fs from "fs";

// UNIX/Linux/Mac
// bash$ export ANCHOR_PROVIDER_URL=http://127.0.0.1:8899
// bash$ export ANCHOR_WALLET=artifacts/id.json

async function main() {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const PROGRAM_ID = new anchor.web3.PublicKey("DWDGo2UkBUFZ3VitBfWRBMvRnHr7E2DSh57NK27xMYaB");
  const program = new Program(idl as anchor.Idl, PROGRAM_ID, anchor.getProvider());

  // Deploy this manually with the solana program deploy...
  const lockbox = new anchor.web3.PublicKey("1okwt4nGbpr82kkr6t1767sAenfeZBxUyzJAAaumZRG");
  const lockboxData = new anchor.web3.PublicKey("Gdt3RDEQmw51NCcUJ13tXR6nj9sgKMaZe1Pic8JSRDfb");

  // User wallet is the provider payer
  const userWallet = provider.wallet["payer"];
  console.log("User wallet:", userWallet.publicKey.toBase58());

    // Find a PDA account for the fee collector program
    const [pdaFeeCollectorProgram, bumpFeeCollector] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("fee_collector", "utf-8")], program.programId);
    //let bumpBytes = Buffer.from(new Uint8Array([bumpFeeCollector]));
    console.log("Fee Collector PDA address:", pdaFeeCollectorProgram.toBase58());
    console.log("Fee Collector PDA bump:", bumpFeeCollector);

  let signature = null;

  // !!!!!!!!!!!! Run this first in order to deploy a program and initialize the pdaFeeCollectorProgram
    // Initialize the FeeCollector program
    try {
        signature = await program.methods
          .initialize()
          .accounts(
            {
              collector: pdaFeeCollectorProgram
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

    console.log("Successfully initialized fee collector");
    return;


    // !!!!!!! Set upgrade authority from the deployer of the lockbox to the pdaFeeCollectorProgram programmatically


    // !!!!!!! Run this after the pdaFeeCollectorProgram is deployed and the lockbox's authority is pdaFeeCollectorProgram
    // Set upgrade authority back
    try {
        signature = await program.methods
          .changeUpgradeAuthority()
          .accounts(
            {
              programToUpdateAuthority: lockbox,
              programDataToUpdateAuthority: lockboxData,
              collector: pdaFeeCollectorProgram,
              destination: userWallet.publicKey
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

    console.log("Successfully transferred program back");
}

main();
