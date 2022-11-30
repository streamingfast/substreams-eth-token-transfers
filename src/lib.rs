mod abi;
mod pb;

use pb::transfers::{transfer::Schema, Transfer, Transfers};
use substreams::log;
use substreams::scalar::BigInt;
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
        schema: schema_to_string(Schema::Erc20),
        from: Hex(&event.from).to_string(),
        to: Hex(&event.to).to_string(),
        quantity: event.value.to_string(),
        trx_hash: Hex(hash).to_string(),
        log_index: log_index as u64,

        operator: "".to_string(),
        token_id: "".to_string(),
    }
}

fn new_erc721_transfer(hash: &[u8], log_index: u32, event: ERC721TransferEvent) -> Transfer {
    Transfer {
        schema: schema_to_string(Schema::Erc721),
        from: Hex(&event.from).to_string(),
        to: Hex(&event.to).to_string(),
        quantity: "1".to_string(),
        trx_hash: Hex(hash).to_string(),
        log_index: log_index as u64,
        token_id: event.token_id.to_string(),

        operator: "".to_string(),
    }
}

fn new_erc1155_single_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC1155TransferSingleEvent,
) -> Transfer {
    new_erc1155_transfer(
        hash,
        log_index,
        &event.from,
        &event.to,
        &event.id,
        &event.value,
        &event.operator,
    )
}

fn new_erc1155_batch_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC1155TransferBatchEvent,
) -> Vec<Transfer> {
    if event.ids.len() != event.values.len() {
        log::info!("There is a different count for ids ({}) and values ({}) in transaction {} for log at block index {}, ERC1155 spec says lenght should match, ignoring the log completely for now",
            event.ids.len(),
            event.values.len(),
            Hex(&hash).to_string(),
            log_index,
        );

        return vec![];
    }

    event
        .ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let value = event.values.get(i).unwrap();

            new_erc1155_transfer(
                hash,
                log_index,
                &event.from,
                &event.to,
                id,
                value,
                &event.operator,
            )
        })
        .collect()
}

fn new_erc1155_transfer(
    hash: &[u8],
    log_index: u32,
    from: &[u8],
    to: &[u8],
    token_id: &BigInt,
    quantity: &BigInt,
    operator: &[u8],
) -> Transfer {
    Transfer {
        schema: schema_to_string(Schema::Erc1155),
        from: Hex(from).to_string(),
        to: Hex(to).to_string(),
        quantity: quantity.to_string(),
        trx_hash: Hex(hash).to_string(),
        log_index: log_index as u64,
        operator: Hex(operator).to_string(),
        token_id: token_id.to_string(),
    }
}

fn schema_to_string(schema: Schema) -> String {
    match schema {
        Schema::Erc20 => "erc20",
        Schema::Erc721 => "erc721",
        Schema::Erc1155 => "erc1155",
    }
    .to_string()
}
