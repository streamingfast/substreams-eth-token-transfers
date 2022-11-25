mod abi;
mod pb;

use pb::transfers::{Erc1155Transfer, Erc20Transfer, Erc721Transfer, Transfer, Transfers};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use abi::erc1155::events::TransferBatch as ERC1155TransferBatchEvent;
use abi::erc1155::events::TransferSingle as ERC1155TransferSingleEvent;
use abi::erc20::events::Transfer as ERC20TransferEvent;
use abi::erc721::events::Transfer as ERC721TransferEvent;

substreams_ethereum::init!();

/// Extracts transfers events from the contract(s)
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, substreams::errors::Error> {
    let transfers: Vec<_> = blk
        .receipts()
        .flat_map(|receipt| {
            let hash = &receipt.transaction.hash;

            receipt.receipt.logs.iter().flat_map(|log| {
                if let Some(event) = ERC20TransferEvent::match_and_decode(log) {
                    return vec![new_erc20_transfer(hash, log.block_index, event)];
                }

                if let Some(event) = ERC721TransferEvent::match_and_decode(log) {
                    return vec![new_erc721_transfer(hash, log.block_index, event)];
                }

                if let Some(event) = ERC1155TransferSingleEvent::match_and_decode(log) {
                    return vec![new_erc1155_single_transfer(hash, log.block_index, event)];
                }

                if let Some(event) = ERC1155TransferBatchEvent::match_and_decode(log) {
                    return new_erc1155_batch_transfer(hash, log.block_index, event);
                }

                vec![]
            })
        })
        .collect();

    Ok(pb::transfers::Transfers { transfers })
}

fn new_erc20_transfer(hash: &[u8], log_index: u32, event: ERC20TransferEvent) -> Transfer {
    Transfer {
        transfer: Some(pb::transfers::transfer::Transfer::Erc20(Erc20Transfer {
            from: Hex(&event.from).to_string(),
            to: Hex(&event.to).to_string(),
            value: event.value.to_string(),
            trx_hash: Hex(hash).to_string(),
            log_index: log_index as u64,
        })),
    }
}

fn new_erc721_transfer(hash: &[u8], log_index: u32, event: ERC721TransferEvent) -> Transfer {
    Transfer {
        transfer: Some(pb::transfers::transfer::Transfer::Erc721(Erc721Transfer {
            from: Hex(&event.from).to_string(),
            to: Hex(&event.to).to_string(),
            token_id: event.token_id.to_string(),
            trx_hash: Hex(hash).to_string(),
            log_index: log_index as u64,
        })),
    }
}

fn new_erc1155_single_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC1155TransferSingleEvent,
) -> Transfer {
    Transfer {
        transfer: Some(pb::transfers::transfer::Transfer::Erc1155(
            Erc1155Transfer {
                operator: Hex(&event.operator).to_string(),
                from: Hex(&event.from).to_string(),
                to: Hex(&event.to).to_string(),
                token_id: event.id.to_string(),
                value: event.value.to_string(),
                trx_hash: Hex(hash).to_string(),
                log_index: log_index as u64,
            },
        )),
    }
}

fn new_erc1155_batch_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC1155TransferBatchEvent,
) -> Vec<Transfer> {
    if event.ids.len() != event.values.len() {
        panic!("There is a different count for ids ({}) and values ({}), there were expected to be the same", event.ids.len(), event.values.len());
    }

    event
        .ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let value = event.values.get(i).unwrap();

            Transfer {
                transfer: Some(pb::transfers::transfer::Transfer::Erc1155(
                    Erc1155Transfer {
                        operator: Hex(&event.operator).to_string(),
                        from: Hex(&event.from).to_string(),
                        to: Hex(&event.to).to_string(),
                        token_id: id.to_string(),
                        value: value.to_string(),
                        trx_hash: Hex(hash).to_string(),
                        log_index: log_index as u64,
                    },
                )),
            }
        })
        .collect()
}
