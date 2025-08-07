import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import idlJson from "./solana_rosca.json"; // This is the JSON file for the program

async function main() {
  const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");

  const wallet = web3.Keypair.generate();

  // Gotta get some SOL to pay for things
  const airdropSignature = await connection.requestAirdrop(
    wallet.publicKey,
    2 * web3.LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(airdropSignature);

  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(wallet), {
    commitment: "confirmed",
  });

  // This is the ID of my program from Anchor.toml
  const programId = new web3.PublicKey("GMhTVRALQqen8Mth7U8KJfv3Kjw41zBxRANAjL35MWfY");

  const idl: anchor.Idl = idlJson as anchor.Idl;

  // Had to add this to make TypeScript work I guess
  // @ts-ignore
  const program = new anchor.Program(idl, programId, provider);

  const group = web3.Keypair.generate();

  // Trying to call the createGroup function
  await program.methods!
    .createGroup(new anchor.BN(1000), 5)
    .accounts({
      group: group.publicKey,
      admin: wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .signers([group])
    .rpc();

  console.log("Group created:", group.publicKey.toBase58());
}

main().catch((error) => {
  console.error("Error:", error);
});