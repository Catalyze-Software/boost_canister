use candid::Principal;
use ic_ledger_types::{
    query_archived_blocks, query_blocks, AccountIdentifier, Block, BlockIndex, GetBlocksArgs,
    Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

use super::store::CATALYZE_MULTI_SIG;

pub struct Ledger {}

impl Ledger {
    // This method checks if the transaction is send and received from the given principal
    pub async fn validate_transaction(
        principal: Principal,
        block_index: BlockIndex,
    ) -> Option<Tokens> {
        // Get the block
        let block = Self::get_block(block_index).await;
        match block {
            Some(block) => {
                // Check if the block has a transaction
                if let Some(operation) = block.transaction.operation {
                    if let ic_ledger_types::Operation::Transfer {
                        from,
                        to,
                        amount,
                        fee: _, // Ignore fee
                    } = operation
                    {
                        if from != Self::principal_to_account_identifier(principal) {
                            return None;
                        }
                        if to
                            != Self::principal_to_account_identifier(
                                Principal::from_text(CATALYZE_MULTI_SIG.to_string()).unwrap(),
                            )
                        {
                            return None;
                        }
                        return Some(amount);
                    } else {
                        // Not a transfer
                        return None;
                    }
                } else {
                    // No operation
                    return None;
                }
            }
            // No block
            None => return None,
        }
    }

    async fn get_block(block_index: BlockIndex) -> Option<Block> {
        let args = GetBlocksArgs {
            start: block_index,
            length: 1,
        };

        match query_blocks(MAINNET_LEDGER_CANISTER_ID, args.clone()).await {
            Ok(blocks_result) => {
                if blocks_result.blocks.len() >= 1 {
                    debug_assert_eq!(blocks_result.first_block_index, block_index);
                    return blocks_result.blocks.into_iter().next();
                }

                if let Some(func) = blocks_result.archived_blocks.into_iter().find_map(|b| {
                    (b.start <= block_index && (block_index - b.start) < b.length)
                        .then(|| b.callback)
                }) {
                    match query_archived_blocks(&func, args).await {
                        Ok(range) => match range {
                            Ok(_range) => return _range.blocks.into_iter().next(),
                            Err(_) => return None,
                        },
                        _ => (),
                    }
                }
            }
            Err(_) => (),
        }

        None
    }

    fn principal_to_account_identifier(principal: Principal) -> AccountIdentifier {
        AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT)
    }
}
