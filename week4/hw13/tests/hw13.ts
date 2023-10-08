import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hw13 } from "../target/types/hw13";
import { assert, expect } from "chai";

describe("hw13", () => {
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Hw13 as Program<Hw13>;
  // Create and initialize myAccount once, outside of the individual tests
  const myAccount = anchor.web3.Keypair.generate();
  
  before(async () => {
    // Initialize myAccount before running the tests
    await program.methods.initialize()
      .accounts({
        myAccount: myAccount.publicKey,
        user: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([myAccount])
      .rpc();
  });

  it("Initializes account with balance 100", async () => {
    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    assert.equal(account.balance, 100);
  });
  
  it("Increments balance by 100", async () => {
    const increment = await program.methods.increment100()
  .accounts({
    myAccount: myAccount.publicKey,
    user: provider.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([myAccount, provider.wallet.payer]) 
  .rpc();

    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    assert.equal(account.balance, 200);
  });
});