import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { RoyaltyNft } from "../target/types/royalty_nft";

const metadataProgramAddress = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const program = anchor.workspace.RoyaltyNft as anchor.Program<RoyaltyNft>;

export function findMetadataPda(mint: PublicKey) {
	const seeds = [
		new Buffer("metadata"),
		new PublicKey(metadataProgramAddress).toBuffer(),
		mint.toBuffer(),
	];
	return PublicKey.findProgramAddressSync(
		seeds,
		new PublicKey(metadataProgramAddress)
	);
}

export function findMasterEditionPda(mint: PublicKey) {
	const seeds = [
		new Buffer("metadata"),
		new PublicKey(metadataProgramAddress).toBuffer(),
		mint.toBuffer(),
		new Buffer("edition"),
	];
	return PublicKey.findProgramAddressSync(
		seeds,
		new PublicKey(metadataProgramAddress)
	);
}

export function findContractStatePda() {
	const seeds = [new Buffer("deauthe")];
	return PublicKey.findProgramAddressSync(
		seeds,
		new PublicKey(program.programId)
	);
}

export function findMintPda(payer: PublicKey) {
	const seeds = [new Buffer("mint"), payer.toBuffer()];
	return PublicKey.findProgramAddressSync(
		seeds,
		new PublicKey(program.programId)
	);
}

export function findtokenAccountPda(payer: PublicKey) {
	const seeds = [new Buffer("mint"), payer.toBuffer()];
	return PublicKey.findProgramAddressSync(
		seeds,
		new PublicKey(program.programId)
	);
}

export function findTokenAccountPda(mint: PublicKey, payer: PublicKey) {
	const seeds = [
		payer.toBuffer(),
		program.programId.toBuffer(),
		mint.toBuffer(),
	];
	return PublicKey.findProgramAddressSync(
		seeds,
		new PublicKey(program.programId)
	);
}
