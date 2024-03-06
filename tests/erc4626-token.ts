import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
} from "@solana/spl-token";
import * as anchor from "@project-serum/anchor";
import { BN, Program } from "@project-serum/anchor";
import { Erc4626Token } from "../target/types/erc4626_token";
import * as assert from "assert";

it("erc4626-token", async () => {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Erc4626Token as Program<Erc4626Token>;
  const tokenProgram = anchor.utils.token.TOKEN_PROGRAM_ID;
  let initialBalance: number;
  const fromWallet = anchor.web3.Keypair.generate();
  // const fromAirdropSignature = await connection.requestAirdrop(fromWallet.publicKey, LAMPORTS_PER_SOL);

  // // Wait for airdrop confirmation
  // await connection.confirmTransaction(fromAirdropSignature);

  const toWallet = anchor.web3.Keypair.generate();

  // Create new token mint
  const mint = await createMint(
    connection,
    fromWallet,
    fromWallet.publicKey,
    null,
    9
  );

  // Get the token account of the fromWallet address
  const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    fromWallet,
    mint,
    fromWallet.publicKey
  );

  // Get the token account of the toWallet address
  const toTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    fromWallet,
    mint,
    toWallet.publicKey
  );

  // Mint 1 new token to the "fromTokenAccount" account we just created
  let signature = await mintTo(
    connection,
    fromWallet,
    mint,
    fromTokenAccount.address,
    fromWallet.publicKey,
    1000000000
  );
  console.log("mint tx:", signature);

  // Transfer the new token to the "toTokenAccount" we just created
  signature = await transfer(
    connection,
    fromWallet,
    fromTokenAccount.address,
    toTokenAccount.address,
    fromWallet.publicKey,
    50
  );

  it("deposit", async () => {
    const deposit = anchor.web3.Keypair.generate();

    await program.rpc.deposit(new BN(12), {
      accounts: {
        from: fromTokenAccount.address,
        owner: program.provider.wallet.publicKey,
        to: toTokenAccount.address,
        deposit: deposit.publicKey,
        tokenProgram: tokenProgram,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [deposit],
    });

    const depositAccount = await program.account.deposit.fetch(deposit.publicKey);

    assert.equal(
        depositAccount.author.toBase58(),
      program.provider.wallet.publicKey.toBase58()
    );
    assert.equal(depositAccount.assets, new BN(12));
    try {
      const balance = await program.provider.connection.getTokenAccountBalance(
        toWallet.publicKey
      );
      initialBalance = balance.value.uiAmount;
    } catch {
      initialBalance = 0;
    }

    const postBalance = (
      await program.provider.connection.getTokenAccountBalance(
        toWallet.publicKey
      )
    ).value.uiAmount;
    assert.equal(
      initialBalance + 12,
      postBalance,
      "Post balance should equal initial plus mint amount"
    );
  });

  it("withdraw", async () => {
    const withdraw = anchor.web3.Keypair.generate();
    const [withdrawPDA, withdrawBump] =
      await PublicKey.findProgramAddress([], program.programId)

    await program.rpc.withdraw(withdrawBump, new BN(12), {
      accounts: {
        from: withdrawPDA,
        owner: withdrawPDA,
        to: toTokenAccount.address,
        withdraw: withdraw.publicKey,
        tokenProgram: tokenProgram,
        systemProgram: program.programId,
      },
    });

    const withdrawAccount = await program.account.withdraw.fetch(withdraw.publicKey);

    assert.equal(
        withdrawAccount.author.toBase58(),
      program.provider.wallet.publicKey.toBase58()
    );
    assert.equal(withdrawAccount.assets, new BN(12));
    try {
      const balance = await program.provider.connection.getTokenAccountBalance(
        toWallet.publicKey
      );
      initialBalance = balance.value.uiAmount;
    } catch {
      initialBalance = 0;
    }

    const postBalance = (
      await program.provider.connection.getTokenAccountBalance(
        toWallet.publicKey
      )
    ).value.uiAmount;
    assert.equal(
      initialBalance + 12,
      postBalance,
      "Post balance should equal initial plus mint amount"
    );
  });
});
