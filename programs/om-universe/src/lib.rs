use anchor_lang::prelude::*;
use mint::*;

mod mint;

declare_id!("64iK3xKi71fxKE6nDR8NZkEjNCFrKWCg521GsBNZ66R8");

#[program]
pub mod om_universe {
  use super::*;

  pub fn mint_genesis_node<'info>(
    ctx: Context<'_, '_, '_, 'info, MintGenesisNode<'info>>,
    data: NftConfigData,
    node_bump: u8,
    gensis_bump: u8
  ) -> ProgramResult {
    // TODO: if the genesis account is minted already, throw error

    // mint contribution as NFT
    mint_nft(
      &ctx.accounts.treasury,
      &ctx.accounts.payer,
      &ctx.accounts.metadata,
      &ctx.accounts.mint,
      &ctx.accounts.mint_authority,
      &ctx.accounts.update_authority,
      &ctx.accounts.master_edition,
      &ctx.accounts.token_metadata_program,
      &ctx.accounts.system_program,
      &ctx.accounts.token_program,
      &ctx.accounts.rent,
      data.name,
      data.symbol,
      data.uri,
    )?;

    // store the node informations
    ctx.accounts.node_account.community_type = String::from("COMMUNITY");
    ctx.accounts.node_account.is_genesis = true;
    ctx.accounts.node_account.parent = None;
    ctx.accounts.node_account.creator = ctx.accounts.payer.key();

    // update genesis information
    ctx.accounts.genesis_node.is_minted = true;
    ctx.accounts.genesis_node.creator = ctx.accounts.payer.key();
    Ok(())
  }

  pub fn mint_node<'info>(
    ctx: Context<'_, '_, '_, 'info, MintNode<'info>>,
    data: NftConfigData,
    bump_node: u8,
  ) -> ProgramResult {
    mint_nft(
      &ctx.accounts.treasury,
      &ctx.accounts.payer,
      &ctx.accounts.metadata,
      &ctx.accounts.mint,
      &ctx.accounts.mint_authority,
      &ctx.accounts.update_authority,
      &ctx.accounts.master_edition,
      &ctx.accounts.token_metadata_program,
      &ctx.accounts.system_program,
      &ctx.accounts.token_program,
      &ctx.accounts.rent,
      data.name,
      data.symbol,
      data.uri,
    )?;

    // store the node informations
    ctx.accounts.node_account.community_type = String::from("COMMUNITY");
    ctx.accounts.node_account.is_genesis = false;
    ctx.accounts.node_account.parent = Some(ctx.accounts.parent_token_account.key());
    ctx.accounts.node_account.creator = ctx.accounts.payer.key();
    Ok(())
  }

  pub fn change_node_type<'info> (
    ctx: Context<'_, '_, '_, 'info, ChangeNodeType<'info>>,
    community_type: String,
  ) -> ProgramResult {
    // TODO: check who has the permission to make this operation happen
    ctx.accounts.graph_node.community_type = community_type;
    Ok(())
  }
}