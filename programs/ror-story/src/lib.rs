use anchor_lang::prelude::*;
use {
  anchor_lang::{
      solana_program::{
        system_program,
        program::{invoke},
      }, AnchorDeserialize, AnchorSerialize, Key,
  },
  spl_token_metadata::{
    instruction::{create_master_edition, create_metadata_accounts, update_metadata_accounts},
  },
};

declare_id!("7PGHGFgZca22mVAps1Px4muWqgJ376ZXnz8Wpo1EgvnC");

#[program]
pub mod ror_story {
  use super::*;

  pub fn mint_story<'info>(
    ctx: Context<'_, '_, '_, 'info, MintStory<'info>>,
    data: NftConfigData,
    bump_node: u8,
    bump_node_child: u8,
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

    // TODO: Add how we store the graph data structure.
    // Our current token is in ctx.accounts.token_account with mint and metadata provided.

    Ok(())
  }

  pub fn change_node_type<'info> (
    ctx: Context<'_, '_, '_, 'info, ChangeNodeType<'info>>,
    community_type: u64,
  ) -> ProgramResult {
    // TODO: check who has the permission to make this operation happen
    ctx.accounts.graph_node.community_type = community_type;
    Ok(())
  }
}

/// instruction input types
#[derive(Accounts)]
#[instruction(data: NftConfigData, bump_node:u8, bump_node_child: u8)]
pub struct MintStory<'info> {
  // mint vars
  #[account(mut, signer)]
  payer: AccountInfo<'info>,
  #[account(mut)]
  treasury: AccountInfo<'info>,
  #[account(mut)]
  metadata: AccountInfo<'info>,
  #[account(mut)]
  mint: AccountInfo<'info>,
  #[account(signer)]
  mint_authority: AccountInfo<'info>,
  #[account(signer)]
  update_authority: AccountInfo<'info>,
  #[account(mut)]
  master_edition: AccountInfo<'info>,
  #[account(mut)]
  token_account: AccountInfo<'info>,

  // story lineage vars
  parent_token: AccountInfo<'info>,
  #[account(mut)]
  parent_node_account: ProgramAccount<'info, GraphNode>,
  #[account(init, seeds=["node_account".as_bytes(), token_account.key().as_ref()], payer=payer, bump=bump_node, space=GRAPH_NODE_STRUCT_SIZE)]
  node_account: ProgramAccount<'info, GraphNode>,
  // #[account(init, seeds=["node_children_account".as_bytes(), token_account.key().as_ref(), ], payer=payer, bump=bump_node_child, space=32)]
  // token_child_account: ProgramAccount<'info, GraphNodeChildren>,

  // system/program vars
  token_metadata_program: AccountInfo<'info>, // spl_token_metadata from crate is actually broken!
  #[account(address = spl_token::id())]
  token_program: AccountInfo<'info>,
  #[account(address = system_program::ID)]
  system_program: AccountInfo<'info>,
  rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ChangeNodeType<'info> {
  #[account(mut)]
  pub graph_node: ProgramAccount<'info, GraphNode>,
}

/// Supporting data structure
#[account]
pub struct GraphNode {
  pub parent: Pubkey,
  pub creator: Pubkey,
  pub num_children: u64,
  pub community_type: u64,
}

#[account]
pub struct GraphNodeChildren {
  pub child: Pubkey
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct NftConfigData {
  pub name: String,
  pub symbol: String,
  pub uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Creator {
  pub address: Pubkey,
  pub verified: bool,
  pub share: u8,
}

pub const GRAPH_NODE_STRUCT_SIZE: usize =  32 + // creator
32 + // parent
64 + // num_children
64   // community type
;

pub const GRAPH_NODE_CHILDE_STRUCT_SIZE: usize = 32;