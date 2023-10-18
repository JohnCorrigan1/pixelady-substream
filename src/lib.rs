mod abi;
mod pb;
use hex_literal::hex;
use pb::eth::erc721::v1 as erc721;
use substreams::{key, prelude::*};
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

// Bored Ape Club Contract
// const TRACKED_CONTRACT: [u8; 20] = hex!("bc4ca0eda7647a8ab7c2061c2e118a18a936f13d");
// pixelady
const TRACKED_CONTRACT: [u8; 20] = hex!("8Fc0D90f2C45a5e7f94904075c952e0943CFCCfd");
const NULL_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
//const TRACKED_CONTRACT: [u8; 20] = hex!("ED5AF388653567Af2F388E6224dC7C4b3241C544");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Option<erc721::Transfers>, substreams::errors::Error> {
    let transfers: Vec<_> = blk
        .events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT])
        .filter_map(|(transfer, log)| {
            if transfer.from == NULL_ADDRESS {
                substreams::log::info!("NFT Transfer seen");

                Some(erc721::Transfer {
                    trx_hash: Hex::encode(&log.receipt.transaction.hash),
                    from: Hex::encode(&transfer.from),
                    to: Hex::encode(&transfer.to),
                    token_id: transfer.token_id.to_u64(),
                    ordinal: log.block_index() as u64,
                })
            } else {
                None
            }
        })
        .collect();
    if transfers.len() == 0 {
        return Ok(None);
    }

    Ok(Some(erc721::Transfers { transfers }))
}

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_mints(blk: eth::Block) -> Result<Option<erc721::Mints>, substreams::errors::Error> {
    let mints: Vec<_> = blk
        .events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT])
        .filter_map(|(mint, _log)| {
            if mint.from == NULL_ADDRESS {
                substreams::log::info!("A juicer was minted!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

                Some(erc721::Mint {
                    to: Hex::encode(&mint.to),
                    token_id: mint.token_id.to_u64(),
                    trx_hash: Hex::encode(&blk.hash),
                })
            } else {
                None
            }
        })
        .collect();
    if mints.len() == 0 {
        return Ok(None);
    }

    Ok(Some(erc721::Mints { mints }))
}

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
///
/*
#[substreams::handlers::store]
fn store_mints(transfers: erc721::Mints, s: StoreAddInt64) {
    log::info!("NFT holders state builder");
    for transfer in transfers.mints {
        log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
        s.add(transfer.token_id, generate_key(&transfer.to), -1);
    }
} */

#[substreams::handlers::map]
fn db_out(
    _clock: substreams::pb::substreams::Clock,
    transfers: erc721::Mints,
    //    _owner_deltas: Deltas<DeltaInt64>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    for transfer in transfers.mints {
        log::info!("Juicer number: {}", transfer.token_id);
        tables
            .create_row("mints", format!("{}", &transfer.token_id))
            .set("trx_hash", transfer.trx_hash)
            .set("to_address", transfer.to)
            .set("token_id", transfer.token_id);
    }

    /*for delta in owner_deltas.into_iter() {
        let holder = key::segment_at(&delta.key, 1);
        let contract = key::segment_at(&delta.key, 2);

        tables
            .create_row("owner_count", format!("{}-{}", contract, holder))
            .set("contract", contract)
            .set("holder", holder)
            .set("balance", delta.new_value)
            .set("block_number", clock.number);
    }*/

    Ok(tables.to_database_changes())
}

//fn generate_key(holder: &String) -> String {
//   return format!("total:{}:{}", holder, Hex(TRACKED_CONTRACT));
//}
