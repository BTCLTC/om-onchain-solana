use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

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
#[instruction(data: NftConfigData, bump_node:u8)]
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

  // story lineage and data vars
  #[account(init, seeds=["node_account".as_bytes(), token_account.key().as_ref()], payer=payer, bump=bump_node, space=GRAPH_NODE_STRUCT_SIZE)]
  pub node_account: ProgramAccount<'info, GraphNode>,

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
