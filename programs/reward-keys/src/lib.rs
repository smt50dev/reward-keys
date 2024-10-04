use anchor_lang::{
        prelude::*,
        solana_program
    };

use solana_security_txt::security_txt;

declare_id!("65BeAsYduu3X91yWbF9TQFj6EqD5TpqFyfWMEkrpdNZv");

fn find_tier_for_value(tiers: &[u64], target_value: u64) -> usize {
    tiers.iter()
        .position(|&value| value >= target_value)
        .unwrap_or_else(|| tiers.len())
}

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "SMT50 Reward Keys Program",
    project_url: "https://smt50.io",
    contacts: "email:info@smt50.io, twitter:@Index_SMT50, telegram:@smt_50",
    policy: "https://github.com/smt50dev/reward-keys/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/smt50dev/reward-keys"
}

#[program]
pub mod reward_keys {

    use super::*;

    const TIERS: [u64; 30] = [0, 600, 1300, 2100, 3000, 4000, 5100, 6300, 7600, 9000, 10500, 12100, 13800, 15600, 17500, 19500, 21550, 23650, 25800, 28000, 30250, 32550, 34900, 37300, 39750, 42250, 44800, 47400, 50050, 52750];
    const PRICES: [u64; 30] = [40, 43, 45, 48, 51, 53, 56, 59, 61, 64, 67, 69, 72, 75, 77, 80, 83, 85, 88, 91, 93, 96, 99, 101, 104, 107, 113, 120, 127, 130];

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Counter init
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter;
        msg!(
            "Counter account initialized! Current Count: {}",
            counter.value
        );


        Ok(())
    }

    pub fn create_key(ctx: Context<CreateKey>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        let value = counter.value;
        
        // Deriving values
        let tier = find_tier_for_value(&TIERS, value.checked_add(1).unwrap());
        // Changing power to 10^9
        let amount = PRICES[tier.checked_sub(1).unwrap()] * 10000000;
        
        if counter.value.checked_add(1).unwrap() < 55500 {
            // Setting counter value
            counter.value = counter.value.checked_add(1).unwrap();

            let temp_amount1 = amount.checked_mul(85).unwrap().checked_div(100).unwrap();
            // SOL transfer to Reward Pool
            let ix1 = solana_program::system_instruction::transfer(
                ctx.accounts.owner.key, 
                ctx.accounts.to_rewardpool.key,
                temp_amount1
            );
            solana_program::program::invoke(
                &ix1, 
                &[
                    ctx.accounts.owner.to_account_info(),
                    ctx.accounts.to_rewardpool.to_account_info()
                ]
            )?;

            let temp_amount2 = amount.checked_div(10).unwrap();
            // SOL transfer to Team Pool
            let ix2 = solana_program::system_instruction::transfer(
                ctx.accounts.owner.key, 
                ctx.accounts.to_project.key,
                temp_amount2
            );
            solana_program::program::invoke(
                &ix2, 
                &[
                    ctx.accounts.owner.to_account_info(),
                    ctx.accounts.to_project.to_account_info()
                ]
            )?;

            let temp_amount3 = amount.checked_sub(temp_amount1).unwrap().checked_sub(temp_amount2).unwrap();
            // SOL transfer to Referral Pool
            let ix3 = solana_program::system_instruction::transfer(
                ctx.accounts.owner.key, 
                ctx.accounts.to_referral.key,
                temp_amount3
            );
            solana_program::program::invoke(
                &ix3, 
                &[
                    ctx.accounts.owner.to_account_info(),
                    ctx.accounts.to_referral.to_account_info()
                ]
            )?;

            // Deriving Reward Key PDA
            let reward_key = &mut ctx.accounts.reward_key;

            // Mutating/creating properties
            reward_key.owner = ctx.accounts.owner.key();
            reward_key.id = value;
            reward_key.tier = tier as u8;
        }

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + Counter::INIT_SPACE,
        seeds = [
            b"counter"
        ],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateKey<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        address = pubkey!("GLcfadqxxNTgKYctuaV5R1qNQeyh8xSgq2Q4up4tQQ9J")
    )]
    /// CHECK: Reward Pool account
    pub to_rewardpool: AccountInfo<'info>,
    #[account(
        mut,
        address = pubkey!("3TUNTEieyEH5WkVnS8mtYZQ5vZf3vmddTaYyu1m3imCa")
    )]
    /// CHECK: Referral Pool account
    pub to_referral: AccountInfo<'info>,
    #[account(
        mut,
        address = pubkey!("HkWjhxzYfH8xXsTVF5Rj1FRb4Z28hoGpPP63DKb71jjo")
    )]
    /// CHECK: Owner account
    pub to_project: AccountInfo<'info>,
    #[account(
        init,
        seeds = [b"key", counter.value.to_le_bytes().as_ref()],
        bump,
        payer = owner,
        space = 8 + RewardKey::INIT_SPACE,
    )]
    pub reward_key: Account<'info, RewardKey>,
    #[account(
        mut,
        seeds = [b"counter"],
        bump = counter.bump
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub value: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct RewardKey {
    pub owner: Pubkey,
    pub id: u64,
    pub tier: u8,
}

#[error_code]
pub enum Errors {
    #[msg("You may not mint more than 10 keys!")]
    TooManyKeys
}
