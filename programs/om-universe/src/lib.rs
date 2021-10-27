use anchor_lang::prelude::*;
use mint::*;
use {
  anchor_lang::{
      solana_program::program::invoke, AnchorDeserialize, AnchorSerialize, Key,
  },
  spl_token_metadata::{
    instruction::{create_master_edition, create_metadata_accounts, update_metadata_accounts},
  },
};

mod mint;

declare_id!("64iK3xKi71fxKE6nDR8NZkEjNCFrKWCg521GsBNZ66R8");

#[program]
pub mod om_universe {
  use super::*;

  pub fn mint_genesis_node<'info>(
    ctx: Context<'_, '_, '_, 'info, MintGenesisNode<'info>>,
    data: NftConfigData,
    bump_node: u8,
  ) -> ProgramResult {
    // TODO: mint the gensis node
    
    Ok(())
  }

  pub fn mint_node<'info>(
    ctx: Context<'_, '_, '_, 'info, MintNode<'info>>,
    data: NftConfigData,
    bump_node: u8,
  ) -> ProgramResult {
    let creators: Vec<spl_token_metadata::state::Creator> =
      vec![spl_token_metadata::state::Creator {
      address: ctx.accounts.treasury.key(),
        verified: false,
        share: 3,
      }, spl_token_metadata::state::Creator {
        address: ctx.accounts.payer.key(),
        verified: false,
        share: 97
      }
    ];

    let metadata_infos = vec![
      ctx.accounts.metadata.clone(),
      ctx.accounts.mint.clone(),
      ctx.accounts.mint_authority.clone(),
      ctx.accounts.update_authority.clone(),
      ctx.accounts.payer.clone(),
      ctx.accounts.token_metadata_program.clone(),
      ctx.accounts.token_program.clone(),
      ctx.accounts.system_program.clone(),
      ctx.accounts.rent.to_account_info().clone(),
    ];

    let master_edition_infos = vec![
      ctx.accounts.master_edition.clone(),
      ctx.accounts.mint.clone(),
      ctx.accounts.mint_authority.clone(),
      ctx.accounts.update_authority.clone(),
      ctx.accounts.payer.clone(),
      ctx.accounts.metadata.clone(),
      ctx.accounts.token_metadata_program.clone(),
      ctx.accounts.token_program.clone(),
      ctx.accounts.system_program.clone(),
      ctx.accounts.rent.to_account_info().clone(),
    ];

    let create_meta_ix = create_metadata_accounts(
      *ctx.accounts.token_metadata_program.key,
      *ctx.accounts.metadata.key,
      *ctx.accounts.mint.key,
      *ctx.accounts.mint_authority.key,
      *ctx.accounts.payer.key,
      *ctx.accounts.update_authority.key,
      data.name,
      data.symbol,
      data.uri,
      Some(creators),
      0,
      true,
      false,
    );
    invoke(
      &create_meta_ix,
      metadata_infos.as_slice(),
    )?;

    invoke(
      &create_master_edition(
        *ctx.accounts.token_metadata_program.key,
        *ctx.accounts.master_edition.key,
        *ctx.accounts.mint.key,
        *ctx.accounts.update_authority.key,
        *ctx.accounts.mint_authority.key,
        *ctx.accounts.metadata.key,
        *ctx.accounts.payer.key,
        Some(0),
      ),
      master_edition_infos.as_slice(),
    )?;
    
    invoke(
      &update_metadata_accounts(
        *ctx.accounts.token_metadata_program.key,
        *ctx.accounts.metadata.key,
        *ctx.accounts.update_authority.key,
        Some(ctx.accounts.update_authority.key()),
        None,
        Some(true),
      ),
      &[
        ctx.accounts.update_authority.clone(),
        ctx.accounts.token_metadata_program.clone(),
        ctx.accounts.metadata.clone(),
      ],
    )?;

    // Store graph data structure
    ctx.accounts.node_account.community_type = String::from("node_account");
    ctx.accounts.node_account.is_genesis = false;
    ctx.accounts.node_account.parent = Some(ctx.accounts.parent_token_account.key());

    // TODO: figure out how to store graph data

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

fn mint_nft<'a>(
  treasury: AccountInfo<'a>,
  payer: AccountInfo<'a>,
  metadata: AccountInfo<'a>,
  mint: AccountInfo<'a>,
  mint_authority: AccountInfo<'a>,
  update_authority: AccountInfo<'a>,
  master_edition: AccountInfo<'a>,
  token_metadata_program: AccountInfo<'a>,
  system_program: AccountInfo<'a>,
  token_program: AccountInfo<'a>,
  rent: Sysvar<'a, Rent>,
  name: String,
  symbol: String,
  uri: String
) -> ProgramResult {
  let creators: Vec<spl_token_metadata::state::Creator> =
      vec![spl_token_metadata::state::Creator {
      address: treasury.key(),
        verified: false,
        share: 3,
      }, spl_token_metadata::state::Creator {
        address: payer.key(),
        verified: false,
        share: 97
      }
    ];

    let metadata_infos = vec![
      metadata.clone(),
      mint.clone(),
      mint_authority.clone(),
      update_authority.clone(),
      payer.clone(),
      token_metadata_program.clone(),
      token_program.clone(),
      system_program.clone(),
      rent.to_account_info().clone(),
    ];

    let master_edition_infos = vec![
      master_edition.clone(),
      mint.clone(),
      mint_authority.clone(),
      update_authority.clone(),
      payer.clone(),
      metadata.clone(),
      token_metadata_program.clone(),
      token_program.clone(),
      system_program.clone(),
      rent.to_account_info().clone(),
    ];

    let create_meta_ix = create_metadata_accounts(
      token_metadata_program.key(),
      metadata.key(),
      mint.key(),
      mint_authority.key(),
      payer.key(),
      update_authority.key(),
      name,
      symbol,
      uri,
      Some(creators),
      0,
      true,
      false,
    );
    invoke(
      &create_meta_ix,
      metadata_infos.as_slice(),
    )?;

    invoke(
      &create_master_edition(
        token_metadata_program.key(),
        master_edition.key(),
        mint.key(),
        update_authority.key(),
        mint_authority.key(),
        metadata.key(),
        payer.key(),
        Some(0),
      ),
      master_edition_infos.as_slice(),
    )?;
    
    invoke(
      &update_metadata_accounts(
        token_metadata_program.key(),
        metadata.key(),
        update_authority.key(),
        Some(update_authority.key()),
        None,
        Some(true),
      ),
      &[
        update_authority.clone(),
        token_metadata_program.clone(),
        metadata.clone(),
      ],
    )?;

    Ok(())
}