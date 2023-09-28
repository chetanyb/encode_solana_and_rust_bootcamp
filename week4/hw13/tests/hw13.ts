import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hw13 } from "../target/types/hw13";

describe("hw13", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Hw13 as Program<Hw13>;

  it("Initializes account with balance 100", async () => {
    const myAccount = anchor.web3.Keypair.generate();

    await program.rpc.initialize({
      accounts: {
        myAccount: myAccount.publicKey,
        user: myAccount.publicKey, 
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    assert.equal(account.balance, 100);
  });
});
