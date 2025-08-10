use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};

declare_id!("7QJCkDU8eEGtarKDNNbLgFCZLfp3QJuxJ8gkp1iLbSTc"); // 替换为你的程序ID

#[program]
pub mod dlmm_swapper {
    use super::*;

    // 执行代币交换
    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        // 1. 转移输入代币到临时账户
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_input_account.to_account_info(),
                to: ctx.accounts.temp_input_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount_in)?;

        // 2. 构建DLMM交换指令
        let swap_instruction = swap_dlmm(
            ctx.accounts.dlmm_program.key(),
            ctx.accounts.pool_state.key(),
            ctx.accounts.temp_input_account.key(),
            ctx.accounts.temp_output_account.key(),
            ctx.accounts.pool_input_vault.key(),
            ctx.accounts.pool_output_vault.key(),
            amount_in,
            min_amount_out,
        )?;

        // 3. 执行DLMM交换 (使用签名PDA)
        let bump = ctx.bumps.temp_account_signer;
        // 正确构造签名种子 (类型修复)
        let signer_seeds: &[&[&[u8]]] = &[&[b"temp_signer".as_ref(), &[bump]]];

        invoke_signed(
            &swap_instruction,
            &[
                ctx.accounts.dlmm_program.to_account_info(),
                ctx.accounts.pool_state.to_account_info(),
                ctx.accounts.temp_input_account.to_account_info(),
                ctx.accounts.temp_output_account.to_account_info(),
                ctx.accounts.pool_input_vault.to_account_info(),
                ctx.accounts.pool_output_vault.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
            signer_seeds, // 正确传递签名
        )?;

        // 4. 将输出代币转回用户
        let transfer_out_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.temp_output_account.to_account_info(),
                to: ctx.accounts.user_output_account.to_account_info(),
                authority: ctx.accounts.temp_account_signer.to_account_info(),
            },
            signer_seeds, // 使用相同的签名
        );
        token::transfer(transfer_out_ctx, min_amount_out)?;

        Ok(())
    }
}

// 构建DLMM交换指令
fn swap_dlmm(
    program_id: Pubkey,
    pool_state: Pubkey,
    input_token: Pubkey,
    output_token: Pubkey,
    input_vault: Pubkey,
    output_vault: Pubkey,
    amount_in: u64,
    min_amount_out: u64,
) -> Result<Instruction> {
    let accounts = vec![
        AccountMeta::new_readonly(pool_state, false),
        AccountMeta::new(input_token, false),
        AccountMeta::new(output_token, false),
        AccountMeta::new(input_vault, false),
        AccountMeta::new(output_vault, false),
        AccountMeta::new_readonly(token::ID, false),
    ];

    let data = DlmmInstruction::Swap {
        amount_in,
        min_amount_out,
    }.pack();

    Ok(Instruction {
        program_id,
        accounts,
        data,
    })
}

// DLMM指令数据结构
#[derive(AnchorSerialize, AnchorDeserialize)]
enum DlmmInstruction {
    Swap { amount_in: u64, min_amount_out: u64 },
}

impl DlmmInstruction {
    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            DlmmInstruction::Swap {
                amount_in,
                min_amount_out,
            } => {
                buf.push(3); // DLMM交换指令号 (根据实际协议调整)
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&min_amount_out.to_le_bytes());
            }
        }
        buf
    }
}

// 账户上下文
#[derive(Accounts)]
pub struct Swap<'info> {
    // 用户信息
    #[account(mut)]
    pub user: Signer<'info>,

    // 代币程序
    pub token_program: Program<'info, Token>,

    // DLMM程序
    /// CHECK: 安全验证在调用时处理
    pub dlmm_program: AccountInfo<'info>,

    // 池子状态账户
    /// CHECK: 安全验证在调用时处理
    pub pool_state: AccountInfo<'info>,

    // 用户输入代币账户 (e.g. wSOL)
    #[account(mut)]
    pub user_input_account: Account<'info, TokenAccount>,

    // 用户输出代币账户 (e.g. AiOShi)
    #[account(mut)]
    pub user_output_account: Account<'info, TokenAccount>,

    // 池子输入代币保险库
    /// CHECK: 安全验证在调用时处理
    #[account(mut)]
    pub pool_input_vault: AccountInfo<'info>,

    // 池子输出代币保险库
    /// CHECK: 安全验证在调用时处理
    #[account(mut)]
    pub pool_output_vault: AccountInfo<'info>,

    // 临时输入代币账户 (PDA)
    #[account(
        mut,
        seeds = [b"temp_input", user.key().as_ref()],
        bump
    )]
    pub temp_input_account: Account<'info, TokenAccount>,

    // 临时输出代币账户 (PDA)
    #[account(
        mut,
        seeds = [b"temp_output", user.key().as_ref()],
        bump
    )]
    pub temp_output_account: Account<'info, TokenAccount>,

    // 临时账户签名者 (PDA)
    /// CHECK: 该PDA账户仅用作临时操作签名验证
    /// - seeds约束确保只有程序能派生该地址
    /// - bump参数自动验证有效性
    #[account(
        seeds = [b"temp_signer"],
        bump
    )]
    pub temp_account_signer: AccountInfo<'info>,

    // 用于访问bump的系统变量
    /// CHECK: Anchor框架内部使用
    pub system_program: Program<'info, System>,
}
