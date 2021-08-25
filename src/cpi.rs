// use anchor_lang::solana_program;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::entrypoint::ProgramResult;
// use anchor_lang::solana_program::program_error::ProgramError;
// use anchor_lang::solana_program::program_pack::Pack;
// use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{Accounts, CpiContext, ToAccountInfos};
use solana_program::instruction::AccountMeta;
// use std::ops::Deref;

pub fn deposit_reserve_liquidity<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, DepositReserveLiquidity<'info>>,
    liquidity_amount: u64,
) -> ProgramResult {
    let ix = spl_token_lending::instruction::deposit_reserve_liquidity(
        // DEVNET, CHANGE IF USING MAINNET
        solend_devnet::ID,
        liquidity_amount,
        *ctx.accounts.source_liquidity.key,
        *ctx.accounts.destination_collateral_account.key,
        *ctx.accounts.reserve_account.key,
        *ctx.accounts.reserve_liquidity_supply.key,
        *ctx.accounts.reserve_collateral_mint.key,
        *ctx.accounts.lending_market_account.key,
        *ctx.accounts.lending_market_authority.key,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        &ctx.signer_seeds,
    )?;

    Ok(())
}

pub fn redeem_reserve_collateral<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, RedeemReserveCollateral<'info>>,
    collateral_amount: u64,
) -> ProgramResult {
    let ix = spl_token_lending::instruction::redeem_reserve_collateral(
        solend_devnet::ID,
        collateral_amount,
        *ctx.accounts.source_collateral.key,
        *ctx.accounts.destination_liquidity.key,
        *ctx.accounts.refreshed_reserve_account.key,
        *ctx.accounts.reserve_collateral_mint.key,
        *ctx.accounts.reserve_liquidity.key,
        *ctx.accounts.lending_market.key,
        *ctx.accounts.lending_market_authority.key,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        &ctx.signer_seeds,
    )?;

    Ok(())
}

pub fn refresh_reserve<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, RefreshReserve<'info>>,
) -> ProgramResult {
    let ix = spl_token_lending::instruction::refresh_reserve(
        solend_devnet::ID,
        *ctx.accounts.reserve.key,
        *ctx.accounts.pyth_reserve_liquidity_oracle.key,
        *ctx.accounts.switchboard_reserve_liquidity_oracle.key,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        &ctx.signer_seeds,
    )?;

    Ok(())
}

pub fn flash_loan<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, FlashLoan<'info>>,
    amount: u64,
) -> ProgramResult {
    let receiver_accounts: Vec<AccountMeta> = Vec::new();

    // Write logic to form AccountMeta for all receiver accounts and
    // push into receiver_accounts

    let ix = spl_token_lending::instruction::flash_loan(
        solend_devnet::ID,
        amount,
        *ctx.accounts.source_liquidity.key,
        *ctx.accounts.destination_liquidity.key,
        *ctx.accounts.reserve.key,
        *ctx.accounts.flash_loan_fee_receiver.key,
        *ctx.accounts.host_fee_receiver.key,
        *ctx.accounts.lending_market.key,
        *ctx.accounts.flask_loan_receiver.key,
        receiver_accounts,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    // Token account for asset to deposit into reserve
    pub source_liquidity: AccountInfo<'info>,
    // Token account for reserve collateral token
    pub destination_collateral_account: AccountInfo<'info>,
    // Reserve state account
    pub reserve_account: AccountInfo<'info>,
    // Token mint for reserve collateral token
    pub reserve_collateral_mint: AccountInfo<'info>,
    // Reserve liquidity supply SPL token account
    pub reserve_liquidity_supply: AccountInfo<'info>,
    // Lending market account
    pub lending_market_account: AccountInfo<'info>,
    // Lending market authority (PDA)
    pub lending_market_authority: AccountInfo<'info>,
    // Transfer authority for accounts 1 and 2
    pub transfer_authority: AccountInfo<'info>,
    // Clock
    pub clock: AccountInfo<'info>,
    // Token program ID
    pub token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemReserveCollateral<'info> {
    // Source token account for reserve collateral token
    pub source_collateral: AccountInfo<'info>,
    // Destination liquidity token account
    pub destination_liquidity: AccountInfo<'info>,
    // Refreshed reserve account
    pub refreshed_reserve_account: AccountInfo<'info>,
    // Reserve collateral mint account
    pub reserve_collateral_mint: AccountInfo<'info>,
    // Reserve liquidity supply SPL Token account.
    pub reserve_liquidity: AccountInfo<'info>,
    // Lending market account
    pub lending_market: AccountInfo<'info>,
    // Lending market authority - PDA
    pub lending_market_authority: AccountInfo<'info>,
    // User transfer authority
    pub user_transfer_authority: AccountInfo<'info>,
    // Clock
    pub clock: AccountInfo<'info>,
    // Token program ID
    pub token_program_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RefreshReserve<'info> {
    // Reserve account
    pub reserve: AccountInfo<'info>,
    // Pyth reserve liquidity oracle
    // Must be the pyth price account specified in InitReserve
    pub pyth_reserve_liquidity_oracle: AccountInfo<'info>,
    // Switchboard Reserve liquidity oracle account
    // Must be the switchboard price account specified in InitReserve
    pub switchboard_reserve_liquidity_oracle: AccountInfo<'info>,
    // Clock
    pub clock: AccountInfo<'info>,
}

/// Accounts expected by this instruction:
///
///   0. `[writable]` Source liquidity token account.
///                     Minted by reserve liquidity mint.
///                     Must match the reserve liquidity supply.
///   1. `[writable]` Destination liquidity token account.
///                     Minted by reserve liquidity mint.
///   2. `[writable]` Reserve account.
///   3. `[writable]` Flash loan fee receiver account.
///                     Must match the reserve liquidity fee receiver.
///   4. `[writable]` Host fee receiver.
///   5. `[]` Lending market account.
///   6. `[]` Derived lending market authority.
///   7. `[]` Token program id.
///   8. `[]` Flash loan receiver program id.
///             Must implement an instruction that has tag of 0 and a signature of `(amount: u64)`
///             This instruction must return the amount to the source liquidity account.
///   .. `[any]` Additional accounts expected by the receiving program's `ReceiveFlashLoan` instruction.
///
///   The flash loan receiver program that is to be invoked should contain an instruction with
///   tag `0` and accept the total amount (including fee) that needs to be returned back after
///   its execution has completed.
///
///   Flash loan receiver should have an instruction with the following signature:
///
///   0. `[writable]` Source liquidity (matching the destination from above).
///   1. `[writable]` Destination liquidity (matching the source from above).
///   2. `[]` Token program id
///   .. `[any]` Additional accounts provided to the lending program's `FlashLoan` instruction above.
///   ReceiveFlashLoan {
///       // Amount that must be repaid by the receiver program
///       amount: u64
///   }
#[derive(Accounts)]
pub struct FlashLoan<'info> {
    // Source liquidity token account
    pub source_liquidity: AccountInfo<'info>,
    // Destination liquidity token account - same mint as source liquidity
    pub destination_liquidity: AccountInfo<'info>,
    // Reserve account
    pub reserve: AccountInfo<'info>,
    // Flash loan fee receiver account
    pub flash_loan_fee_receiver: AccountInfo<'info>,
    // Host fee receiver
    pub host_fee_receiver: AccountInfo<'info>,
    // Lending market account
    pub lending_market: AccountInfo<'info>,
    // Derived lending market authority - PDA
    pub derived_lending_market_authority: AccountInfo<'info>,
    // Token program ID
    pub token_program_id: AccountInfo<'info>,
    // Flash loan program receiver ID
    pub flask_loan_receiver: AccountInfo<'info>,
    // OPTIONAL: ADD ANY ADDITIONAL ACCOUNTS THAT MAY BE EXPECTED BY THE
    // RECEIVER'S FLASHLOAN INSTRUCTION
}

// Mainnet ID of the Solend protocol program
pub mod solend_mainnet {
    solana_program::declare_id!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
}

// Devnet ID of the Solend protocol program
pub mod solend_devnet {
    solana_program::declare_id!("ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx");
}
