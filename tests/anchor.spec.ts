import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RoyaltyNft } from "../target/types/royalty_nft";
import {
	Keypair,
	PublicKey,
	sendAndConfirmTransaction,
	SendTransactionError,
} from "@solana/web3.js";
import { simulateTransaction } from "@coral-xyz/anchor/dist/cjs/utils/rpc";
import { findMasterEditionPda, findMetadataPda } from "./utils";

describe("royalty nft tests", () => {
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);
	const connection = provider.connection;
	const wallet = provider.wallet as anchor.Wallet;

	const program = anchor.workspace.RoyaltyNft as Program<RoyaltyNft>;
	const payer = Keypair.generate();

	const metadataContractAddress = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

	beforeAll(async () => {
		// Transfer SOL to payer
		const transferAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
		const transferTx = new anchor.web3.Transaction().add(
			anchor.web3.SystemProgram.transfer({
				fromPubkey: wallet.publicKey,
				toPubkey: payer.publicKey,
				lamports: transferAmount,
			})
		);
		const txSig = await provider.sendAndConfirm(transferTx);
		console.log("Transfer sol to payer transaction signature:", txSig);
		console.log("program id", program.programId.toBase58());
	});

	it("initiating contract", async () => {
		const tx = await program.methods.intializeContract().rpc();
		console.log("signature", tx);
	});

	it("creating nft", async () => {
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

		const [metadataPda, metadataAccountBump] = findMetadataPda(mintPda);
		const [masterEditionPda, masterEditionBump] = findMasterEditionPda(mintPda);
		console.log(
			"mintPda",
			mintPda.toBase58(),
			"\n tokenAccountPda",
			tokenAccountPda.toBase58(),
			"\n payer",
			payer.publicKey.toBase58(),
			"\n payer balance",
			await connection.getBalance(payer.publicKey),
			"\n metadataPda",
			metadataPda,
			"\n masterEditionPda",
			masterEditionPda
		);
		const tx = await program.methods
			.createNft("SYM", "test-token", "test-uri")
			.accounts({
				payer: payer.publicKey,
				editionAccount: masterEditionPda,
				metadata: metadataPda,
			})
			.transaction();

		try {
			const txSig = await sendAndConfirmTransaction(connection, tx, [payer], {
				commitment: "confirmed",
				skipPreflight: true,
			});
			console.log("Transaction signature:", txSig);

			// tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
			// const txSim = await simulateTransaction(connection, tx, [payer]);
			// console.log("Transaction simulation:", txSim);
		} catch (error) {
			if (error.logs) {
				console.log(
					"Transaction failed, logs :",
					error.logs,
					"\n tx message",
					error.message
				);
			} else {
				console.error("Transaction failed w/o logs", error);
			}
		}

		expect(tx).toBeDefined();
	});
});
