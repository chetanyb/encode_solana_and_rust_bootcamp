//client code tested on solpg
// Client
console.log("My address:", pg.wallet.publicKey.toString());
const airdropSignature = await pg.connection.requestAirdrop(pg.wallet.publicKey, 9999);
await pg.connection.confirmTransaction(airdropSignature);
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);