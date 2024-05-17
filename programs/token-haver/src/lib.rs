use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;

pub mod tools;

declare_id!("vo65JQC6U8HCDPfoJxRrr17n7RZ5fKdfyjHm3erHJ2V");

#[program]
pub mod token_haver {

    use super::*;

    pub fn create_registrar(ctx: Context<CreateRegistrar>, mints: Vec<Pubkey>) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx, mints)
    }
    pub fn create_voter_weight_record(
        ctx: Context<CreateVoterWeightRecord>,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        log_version();
        instructions::create_voter_weight_record(ctx, governing_token_owner)
    }
    pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::update_voter_weight_record(ctx)
    }
    pub fn configure_mints(ctx: Context<ConfigureMints>, mints: Vec<Pubkey>) -> Result<()> {
        log_version();
        instructions::configure_mints(ctx, mints)
    }
}

fn log_version() {
    // TODO: Check if Anchor allows to log it before instruction is deserialized
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}
