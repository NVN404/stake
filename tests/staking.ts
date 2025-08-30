import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Staking } from "../../target/types/staking";
import { PublicKey, Keypair } from "@solana/web3.js";
import { createMint, mintTo, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { expect } from "chai";

describe("staking", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Staking as Program<Staking>;
  const user = Keypair.generate();
  let mint: PublicKey;
  let userTokenAccount: PublicKey;
  let vault: PublicKey;
  let vaultTokenAccount: PublicKey;
  let stakingAccount: PublicKey;

  before(async () => {
    // Airdrop SOL to user
    await provider.connection.requestAirdrop(user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);

    // Create mock SPL token
    mint = await createMint(
      provider.connection,
      user,
      user.publicKey,
      null,
      9
    );

    // Create user token account
    const userToken = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      user.publicKey
    );
    userTokenAccount = userToken.address;

    // Mint tokens to user
    await mintTo(
      provider.connection,
      user,
      mint,
      userTokenAccount,
      user,
      1000 * 10 ** 9
    );

    // Derive vault and vault token account
    [vault] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), mint.toBuffer()],
      program.programId
    );
    const vaultToken = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      vault,
      true
    );
    vaultTokenAccount = vaultToken.address;

    // Derive staking account
    [stakingAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), user.publicKey.toBuffer(), vault.toBuffer()],
      program.programId
    );
  });

  it("Initializes the vault", async () => {
    await program.methods
      .initialize()
      .accounts({
        signer: user.publicKey,
        vault,
        tokenMint: mint,
        vaultTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([user])
      .rpc();

    const vaultAccount = await program.account.vault.fetch(vault);
    expect(vaultAccount.tokenMint.toBase58()).to.equal(mint.toBase58());
  });

  it("Deposits tokens", async () => {
    const amount = new anchor.BN(100 * 10 ** 9);
    await program.methods
      .deposit(amount)
      .accounts({
        signer: user.publicKey,
        stakingAccount,
        vault,
        vaultTokenAccount,
        userTokenAccount,
        tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([user])
      .rpc();

    const stakingAccountData = await program.account.stakingAccount.fetch(stakingAccount);
    expect(stakingAccountData.balance.toNumber()).to.equal(amount.toNumber());
    expect(stakingAccountData.owner.toBase58()).to.equal(user.publicKey.toBase58());
  });

  it("Withdraws tokens", async () => {
    const amount = new anchor.BN(50 * 10 ** 9);
    await program.methods
      .withdraw(amount)
      .accounts({
        signer: user.publicKey,
        stakingAccount,
        vault,
        vaultTokenAccount,
        userTokenAccount,
        tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    const stakingAccountData = await program.account.stakingAccount.fetch(stakingAccount);
    expect(stakingAccountData.balance.toNumber()).to.equal(50 * 10 ** 9);
  });

  it("Fails to withdraw more than balance", async () => {
    const amount = new anchor.BN(100 * 10 ** 9);
    try {
      await program.methods
        .withdraw(amount)
        .accounts({
          signer: user.publicKey,
          stakingAccount,
          vault,
          vaultTokenAccount,
          userTokenAccount,
          tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err.error.errorMessage).to.include("Insufficient balance");
    }
  });
});