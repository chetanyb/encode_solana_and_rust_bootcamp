import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hw13 } from "../target/types/hw13";

describe("hw13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Hw13 as Program<Hw13>;

  it("Initializes account with balance 100", async () => {
    const myAccount = anchor.web3.Keypair.generate();
    const user = anchor.web3.Keypair.generate(); // Added this line for the user account

    await program.rpc.initialize({
      accounts: {
        myAccount: myAccount.publicKey,
        user: user.publicKey, // Added this line for the user account
        systemProgram: anchor.web3.SystemProgram.programId, // Added this line for the system program
      },
      signers: [myAccount, user], // Added user to the signers
      instructions: [await program.account.myAccount.createInstruction(myAccount)],
    });

    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    expect(account.balance).toEqual(100);
  });
});