import { startAnchor } from "anchor-bankrun";
import {
	Keypair,
	PublicKey,
	SYSVAR_RENT_PUBKEY,
	VersionedTransaction,
} from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { RoyaltyNft } from "../target/types/royalty_nft";
import { Program } from "@coral-xyz/anchor";
import { expect, describe, test } from "@jest/globals";
import * as anchor from "@coral-xyz/anchor";

const contractStateSeed = "contract_state";

describe("royalty_nft", () => {
	test("initiate contract", async () => {
		let context = await startAnchor("./", [], []);
		const client = context.banksClient;

		let provider = new BankrunProvider(context);
		anchor.setProvider(provider);
		let program = anchor.workspace.RoyaltyNft as Program<RoyaltyNft>;
		let payer = provider.wallet as anchor.Wallet;

		const [contractStatePda, contractStateBump] =
			await PublicKey.findProgramAddress(
				[Buffer.from("deauthe")],
				program.programId
			);

		try {
			const tx = await program.methods
				.intializeContract()
				.accountsStrict({
					contractState: contractStatePda,
					systemProgram: anchor.web3.SystemProgram.programId,
					payer: payer.publicKey,
				})
				.rpc();
			console.log("tx", tx);
			console.log("payer account :", payer.publicKey.toBase58());

			expect(tx).toBeDefined();
		} catch (err) {
			console.log("Transaction failed");
			console.log("payer account :", payer.publicKey.toBase58());

			throw err;
		}

		const [mintPda, mintPdaBump] = await PublicKey.findProgramAddress(
			[Buffer.from("mint")],
			program.programId
		);

		const [tokenAccountPda, tokenAccountBump] =
			await PublicKey.findProgramAddress(
				[
					payer.publicKey.toBuffer(),
					program.programId.toBuffer(),
					mintPda.toBuffer(),
				],
				anchor.utils.token.TOKEN_PROGRAM_ID
			);
		console.log("mintPda", mintPda.toBase58());
		console.log("tokenAccountPda", tokenAccountPda.toBase58());

		const blockhash = context.lastBlockhash;
		try {
			const tx = await program.methods
				.createNft("SYM", "test-token", "test-uri")
				.accountsPartial({
					payer: payer.publicKey,
					mint: mintPda,
					tokenAccount: tokenAccountPda,
					contractState: contractStatePda,
					systemProgram: anchor.web3.SystemProgram.programId,
					tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
				})
				.transaction();
			tx.recentBlockhash = blockhash;
			payer.signTransaction(tx);
			client.sendTransaction(tx);
			console.log("create nft transaction", tx);

			expect(tx).toBeDefined();
		} catch (err) {
			throw err;
		}
	});

	test("create_nft", async () => {
		let payer = Keypair.generate();
		let context = await startAnchor("./", [], []);
		const client = context.banksClient;

		let provider = new BankrunProvider(context);
		anchor.setProvider(provider);
		let program = anchor.workspace.RoyaltyNft as Program<RoyaltyNft>;

		const [contractStatePda, contractStateBump] =
			await PublicKey.findProgramAddress(
				[Buffer.from("deauthe")],
				program.programId
			);

		try {
			const tx = await program.methods
				.intializeContract()
				.accountsStrict({
					contractState: contractStatePda,
					systemProgram: anchor.web3.SystemProgram.programId,
					payer: payer.publicKey,
				})
				.signers([payer])
				.simulate();
			console.log("tx", tx);
			console.log("payer account :", payer.publicKey.toBase58());

			expect(tx).toBeDefined();
		} catch (err) {
			console.log("Transaction failed");
			console.log("payer account 2 :", payer.publicKey.toBase58());

			throw err;
		}

		const [royaltyPoolPda, royaltyPoolBump] =
			await PublicKey.findProgramAddress(
				[Buffer.from("royalty_pool")],
				program.programId
			);

		const [mintPda, mintPdaBump] = await PublicKey.findProgramAddress(
			[Buffer.from("mint")],
			program.programId
		);

		const [tokenAccountPda, tokenAccountBump] =
			await PublicKey.findProgramAddress(
				[
					payer.publicKey.toBuffer(),
					program.programId.toBuffer(),
					mintPda.toBuffer(),
				],
				anchor.utils.token.TOKEN_PROGRAM_ID
			);
		console.log("mintPda", mintPda.toBase58());
		console.log("tokenAccountPda", tokenAccountPda.toBase58());
		try {
			const tx = await program.methods
				.createNft("SYM", "test-token", "test-uri")
				.accountsPartial({
					payer: payer.publicKey,
					mint: mintPda,
					tokenAccount: tokenAccountPda,
					contractState: contractStatePda,
					systemProgram: anchor.web3.SystemProgram.programId,
					tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
					royaltyPoolAccount: royaltyPoolPda,
				})
				.signers([payer])
				.rpc({ skipPreflight: true });
			console.log("create nft transaction", tx);

			await anchor.getProvider().connection.confirmTransaction(tx, "processed");
			expect(tx).toBeDefined();
		} catch (err) {
			throw err;
		}
	});
});
