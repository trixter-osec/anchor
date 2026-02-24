use trixter_osec_anchor_lang::solana_program::pubkey::Pubkey;
use trixter_osec_anchor_lang::Result;
use trixter_osec_anchor_lang::{context::CpiContext, Accounts};

pub use spl_memo_interface::instruction as spl_memo;
pub use spl_memo_interface::v3::ID;

pub fn build_memo<'info>(ctx: CpiContext<'_, '_, '_, 'info, BuildMemo>, memo: &[u8]) -> Result<()> {
    let ix = spl_memo::build_memo(
        &ID,
        memo,
        &ctx.remaining_accounts
            .iter()
            .map(|account| account.key)
            .collect::<Vec<_>>(),
    );
    trixter_osec_anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &ctx.remaining_accounts,
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct BuildMemo {}

#[derive(Clone)]
pub struct Memo;

impl trixter_osec_anchor_lang::Id for Memo {
    fn id() -> Pubkey {
        ID
    }
}
