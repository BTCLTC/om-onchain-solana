use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, program::invoke};
use spl_token_metadata::{
  instruction::{create_master_edition, create_metadata_accounts, update_metadata_accounts},
};

#[derive(Accounts)]
#[instruction(data: NftConfigData, bump_node:u8)]
pub struct MintNode<'info> {
  // mint vars
  #[account(mut, signer)]
  pub payer: AccountInfo<'info>,
  #[account(mut)]
  pub treasury: AccountInfo<'info>,
  #[account(mut)]
  pub metadata: AccountInfo<'info>,
  #[account(mut)]
  pub mint: AccountInfo<'info>,
  #[account(signer)]
  pub mint_authority: AccountInfo<'info>,
  #[account(signer)]
  pub update_authority: AccountInfo<'info>,
  #[account(mut)]
  pub master_edition: AccountInfo<'info>,
  #[account(mut)]
  pub token_account: AccountInfo<'info>,

  // story lineage and data vars
  #[account(init, seeds=["node_account".as_bytes(), token_account.key().as_ref()], payer=payer, bump=bump_node, space=GRAPH_NODE_STRUCT_SIZE)]
  pub node_account: ProgramAccount<'info, GraphNode>,
  pub parent_token_account: AccountInfo<'info>,

  // system/program vars
  pub token_metadata_program: AccountInfo<'info>, // spl_token_metadata from crate is actually broken!
  #[account(address = spl_token::id())]
  pub token_program: AccountInfo<'info>,
  #[account(address = system_program::ID)]
  pub system_program: AccountInfo<'info>,
  pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(data: NftConfigData, node_bump:u8, genesis_bump: u8)]
pub struct MintGenesisNode<'info> {
  // mint vars
  #[account(mut, signer)]
  pub payer: AccountInfo<'info>,
  #[account(mut)]
  pub treasury: AccountInfo<'info>,
  #[account(mut)]
  pub metadata: AccountInfo<'info>,
  #[account(mut)]
  pub mint: AccountInfo<'info>,
  #[account(signer)]
  pub mint_authority: AccountInfo<'info>,
  #[account(signer)]
  pub update_authority: AccountInfo<'info>,
  #[account(mut)]
  pub master_edition: AccountInfo<'info>,
  #[account(mut)]
  pub token_account: AccountInfo<'info>,

  // node lineage
  #[account(init, seeds=["node_account".as_bytes(), token_account.key().as_ref()], payer=payer, bump=node_bump, space=GRAPH_NODE_STRUCT_SIZE)]
  pub node_account: ProgramAccount<'info, GraphNode>,
  #[account(init, seeds=["node_account".as_bytes(), "gensis".as_bytes()], payer=payer, bump=genesis_bump, space=1+32)]
  pub genesis_node: ProgramAccount<'info, GraphGenesis>,

  // system/program vars
  pub token_metadata_program: AccountInfo<'info>, // spl_token_metadata from crate is actually broken!
  #[account(address = spl_token::id())]
  pub token_program: AccountInfo<'info>,
  #[account(address = system_program::ID)]
  pub system_program: AccountInfo<'info>,
  pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ChangeNodeType<'info> {
  #[account(mut)]
   pub graph_node: ProgramAccount<'info, GraphNode>,
}

/// Supporting data structure
#[account]
pub struct GraphNode {
  pub parent: Option<Pubkey>, // 32
  pub creator: Pubkey,        // 32
  pub is_genesis: bool,       // 1
  pub community_type: String, // 24 
}

#[account]
pub struct GraphGenesis {
  pub is_minted: bool, // 1,
  pub creator: Pubkey, // 32,
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
1 + // is_genesis
24;   // community type


pub fn mint_nft<'a>(
  treasury: &AccountInfo<'a>,
  payer: &AccountInfo<'a>,
  metadata: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  mint_authority: &AccountInfo<'a>,
  update_authority: &AccountInfo<'a>,
  master_edition: &AccountInfo<'a>,
  token_metadata_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent: &Sysvar<'a, Rent>,
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