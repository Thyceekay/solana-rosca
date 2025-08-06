import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaRosca } from "../target/types/solana_rosca";

// import * as anchor from "@project-serum/anchor";
// import { Program } from "@project-serum/anchor";
// import { SolanaRosca } from "../target/types/solana_rosca";
import assert from "assert";

console.log("Anchor workspace:", Object.keys(anchor.workspace));

describe("solana-rosca", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider); 
  const program = anchor.workspace.SolanaRosca as Program<SolanaRosca>;

  it("Creates a group", async () => {
    const group = anchor.web3.Keypair.generate();
    await program.methods
      .createGroup(new anchor.BN(1000), 5)
      .accounts({
        group: group.publicKey,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([group])
      .rpc();

    const groupAccount = await program.account.group.fetch(group.publicKey);
    assert.equal(groupAccount.contributionAmount.toNumber(), 1000);
    assert.equal(groupAccount.maxParticipants, 5);
    assert.equal(groupAccount.currentWeek, 0);
  });

  it("Joins a group", async () => {
    const group = anchor.web3.Keypair.generate();
    await program.methods
      .createGroup(new anchor.BN(1000), 5)
      .accounts({
        group: group.publicKey,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([group])
      .rpc();

    const participant = anchor.web3.Keypair.generate();
    const user = anchor.web3.Keypair.generate();
    await program.methods
      .joinGroup()
      .accounts({
        group: group.publicKey,
        participant: participant.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([participant, user])
      .rpc();

    const groupAccount = await program.account.group.fetch(group.publicKey);
    assert.equal(groupAccount.participants.length, 1);
  });

});
