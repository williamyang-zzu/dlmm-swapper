// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { DlmmSwapper } from "../target/types/dlmm_swapper";
// import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from "@solana/spl-token";
// import { assert } from "chai";
// // describe("dlmm-swapper", () => {
// //   // Configure the client to use the local cluster.
// //   anchor.setProvider(anchor.AnchorProvider.env());

// //   const program = anchor.workspace.dlmmSwapper as Program<DlmmSwapper>;

// //   it("Is initialized!", async () => {
// //     // Add your test here.
// //     const tx = await program.methods.initialize().rpc();
// //     console.log("Your transaction signature", tx);
// //   });
// // });

// describe("dlmm-swapper", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program = anchor.workspace.DlmmSwapper as Program<DlmmSwapper>;

//   const connection = provider.connection;
//   const wallet = provider.wallet as anchor.Wallet;

//   let userInputMint: anchor.web3.PublicKey;
//   let userOutputMint: anchor.web3.PublicKey;

//   let userInputAccount: anchor.web3.PublicKey;
//   let userOutputAccount: anchor.web3.PublicKey;

//   let tempInputAccount: anchor.web3.PublicKey;
//   let tempOutputAccount: anchor.web3.PublicKey;
//   let tempAccountSigner: anchor.web3.PublicKey;

//   // 伪造 DLMM 相关地址（你应替换为真实主网地址）
//   const dlmmProgram = anchor.web3.Keypair.generate().publicKey;
//   const poolState = anchor.web3.Keypair.generate().publicKey;
//   const poolInputVault = anchor.web3.Keypair.generate().publicKey;
//   const poolOutputVault = anchor.web3.Keypair.generate().publicKey;

//   it("performs swap", async () => {
//     // 1. 创建两个测试mint (tokenA 和 tokenB)
//     userInputMint = await createMint(connection, wallet.payer, wallet.publicKey, null, 6);
//     userOutputMint = await createMint(connection, wallet.payer, wallet.publicKey, null, 6);

//     // 2. 创建用户的 token account
//     userInputAccount = await createAccount(connection, wallet.payer, userInputMint, wallet.publicKey);
//     userOutputAccount = await createAccount(connection, wallet.payer, userOutputMint, wallet.publicKey);

//     // 3. Mint 一些代币到输入账户
//     await mintTo(connection, wallet.payer, userInputMint, userInputAccount, wallet.payer, 1_000_000); // 1 token

//     // 4. 派生 PDA：tempInputAccount, tempOutputAccount, tempAccountSigner
//     const [tempInputPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("temp_input"), wallet.publicKey.toBuffer()],
//       program.programId
//     );
//     const [tempOutputPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("temp_output"), wallet.publicKey.toBuffer()],
//       program.programId
//     );
//     const [tempSignerPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("temp_signer")],
//       program.programId
//     );

//     tempInputAccount = tempInputPDA;
//     tempOutputAccount = tempOutputPDA;
//     tempAccountSigner = tempSignerPDA;

//     // 💡 注意：为了简化测试，我们假设 tempInput/outputAccount 是预先创建好的 TokenAccount
//     // 实际部署中，这两个PDA账户需要由合约或前置脚本先创建并初始化为 token account

//     // 5. 调用 swap instruction
//     await program.methods
//       .swap(new anchor.BN(100_000), new anchor.BN(80_000)) // amount_in, min_amount_out
//       .accounts({
//         user: wallet.publicKey,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         dlmmProgram,
//         poolState,
//         userInputAccount,
//         userOutputAccount,
//         poolInputVault,
//         poolOutputVault,
//         tempInputAccount,
//         tempOutputAccount,
//         tempAccountSigner,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .rpc();

//     console.log("✅ Swap instruction executed");
//   });
// });

import * as anchor from "@coral-xyz/anchor";
// import { DlmmSwapper } from "../target/types/dlmm_swapper";
import DLMM from "@meteora-ag/dlmm";
import { PublicKey } from "@solana/web3.js";

describe("dlmm-swapper", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  const connection = provider.connection;

  it("fetches pool state (lbPair) from Meteora mainnet using DLMM.create", async () => {
    const poolPubkey = new PublicKey("EcRb4AuGBUCndxaZU1YdeEB26gqvCX9tt2YHkRKB3RRT");

    // 使用官方推荐的 create 方法，绑定指定池子
    const dlmmPool = await DLMM.create(connection, poolPubkey);

    // dlmmPool 对象上就直接包含 lbPair 等池状态
    console.log("lbPair baseKey:", dlmmPool.lbPair.baseKey.toBase58());
    console.log("inputVault:", dlmmPool.lbPair.tokenXMint.toBase58());
    console.log("outputVault:", dlmmPool.lbPair.tokenYMint.toBase58());
    console.log("dlmmProgramId:", dlmmPool.program.programId.toBase58());

    // 你可以加断言，确认地址正确
    // anchor.assert.ok(dlmmPool.lbPair.publicKey.equals(poolPubkey));
  });
});


