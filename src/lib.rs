mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const GM_TRACKED_CONTRACT: [u8; 20] = hex!("d46a240a1d63b066340af1d7f1d6359f0f6fd845");

fn map_gm_events(blk: &eth::Block, events: &mut contract::Events) {
    events.gm_blog_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::BlogCreated::match_and_decode(log) {
                        return Some(contract::GmBlogCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            blog_id: event.blog_id.to_string(),
                            blog_link: event.blog_link,
                            categories: event.categories.into_iter().map(|x| x).collect::<Vec<_>>(),
                            owner: event.owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_blog_tippeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::BlogTipped::match_and_decode(log) {
                        return Some(contract::GmBlogTipped {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            blog_id: event.blog_id.to_string(),
                            tipper: event.tipper,
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_blog_vieweds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::BlogViewed::match_and_decode(log) {
                        return Some(contract::GmBlogViewed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: event.id.to_string(),
                            views: event.views.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_news_tippeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::NewsTipped::match_and_decode(log) {
                        return Some(contract::GmNewsTipped {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            blog_id: event.blog_id.to_string(),
                            tipper: event.tipper,
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_news_vieweds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::NewsViewed::match_and_decode(log) {
                        return Some(contract::GmNewsViewed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: event.id.to_string(),
                            views: event.views.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_ownership_transfer_requesteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::OwnershipTransferRequested::match_and_decode(log) {
                        return Some(contract::GmOwnershipTransferRequested {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::GmOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_request_fulfilleds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::RequestFulfilled::match_and_decode(log) {
                        return Some(contract::GmRequestFulfilled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: Vec::from(event.id),
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_request_sents.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::RequestSent::match_and_decode(log) {
                        return Some(contract::GmRequestSent {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            id: Vec::from(event.id),
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_responses.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::Response::match_and_decode(log) {
                        return Some(contract::GmResponse {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            character: event.character,
                            err: event.err,
                            request_id: Vec::from(event.request_id),
                            response: event.response,
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_user_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::UserUpdated::match_and_decode(log) {
                        return Some(contract::GmUserUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            interest: event.interest.into_iter().map(|x| x).collect::<Vec<_>>(),
                            name: event.name,
                            profile_url: event.profile_url,
                            user_address: event.user_address,
                            username: event.username,
                        });
                    }

                    None
                })
        })
        .collect());
    events.gm_news_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == GM_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::gm_contract::events::NewsCreated::match_and_decode(log) {
                        return Some(contract::GmNewsCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            blog_id: event.blog_id.to_string(),
                            blog_link: event.blog_link,
                            categories: event.categories.into_iter().map(|x| x).collect::<Vec<_>>(),
                        });
                    }

                    None
                })
        })
        .collect());
}

fn db_gm_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.gm_blog_createds.iter().for_each(|evt| {
        tables
            .create_row("gm_blog_created", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("blog_link", &evt.blog_link)
            .set_psql_array("categories", evt.categories.clone())
            .set("owner", Hex(&evt.owner).to_string());
    });
    events.gm_blog_tippeds.iter().for_each(|evt| {
        tables
            .create_row("gm_blog_tipped", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("tipper", Hex(&evt.tipper).to_string());
    });
    events.gm_blog_vieweds.iter().for_each(|evt| {
        tables
            .create_row("gm_blog_viewed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", BigDecimal::from_str(&evt.id).unwrap())
            .set("views", BigDecimal::from_str(&evt.views).unwrap());
    });
    events.gm_news_tippeds.iter().for_each(|evt| {
        tables
            .create_row("gm_news_tipped", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("tipper", Hex(&evt.tipper).to_string());
    });
    events.gm_news_vieweds.iter().for_each(|evt| {
        tables
            .create_row("gm_news_viewed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", BigDecimal::from_str(&evt.id).unwrap())
            .set("views", BigDecimal::from_str(&evt.views).unwrap());
    });
    events.gm_ownership_transfer_requesteds.iter().for_each(|evt| {
        tables
            .create_row("gm_ownership_transfer_requested", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.gm_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("gm_ownership_transferred", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.gm_request_fulfilleds.iter().for_each(|evt| {
        tables
            .create_row("gm_request_fulfilled", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", Hex(&evt.id).to_string());
    });
    events.gm_request_sents.iter().for_each(|evt| {
        tables
            .create_row("gm_request_sent", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", Hex(&evt.id).to_string());
    });
    events.gm_responses.iter().for_each(|evt| {
        tables
            .create_row("gm_response", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("character", &evt.character)
            .set("err", Hex(&evt.err).to_string())
            .set("request_id", Hex(&evt.request_id).to_string())
            .set("response", Hex(&evt.response).to_string());
    });
    events.gm_user_updateds.iter().for_each(|evt| {
        tables
            .create_row("gm_user_updated", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
           .set_psql_array("interest", evt.interest.iter().cloned().collect::<Vec<_>>())
            .set("name", &evt.name)
            .set("profile_url", &evt.profile_url)
            .set("user_address", Hex(&evt.user_address).to_string())
            .set("username", &evt.username);
    });
    events.gm_news_createds.iter().for_each(|evt| {
        tables
            .create_row("gm_news_created", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("blog_link", &evt.blog_link)
        .set_psql_array("categories", evt.categories.iter().cloned().collect::<Vec<_>>());
    });
}


fn graph_gm_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.gm_blog_createds.iter().for_each(|evt| {
        tables
            .create_row("gm_blog_created", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("blog_link", &evt.blog_link)
          .set("categories", evt.categories.iter().cloned().collect::<Vec<_>>())
            .set("owner", Hex(&evt.owner).to_string());
    });
    events.gm_blog_tippeds.iter().for_each(|evt| {
        tables
            .create_row("gm_blog_tipped", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("tipper", Hex(&evt.tipper).to_string());
    });
    events.gm_blog_vieweds.iter().for_each(|evt| {
        tables
            .create_row("gm_blog_viewed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", BigDecimal::from_str(&evt.id).unwrap())
            .set("views", BigDecimal::from_str(&evt.views).unwrap());
    });
    events.gm_news_tippeds.iter().for_each(|evt| {
        tables
            .create_row("gm_news_tipped", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("tipper", Hex(&evt.tipper).to_string());
    });
    events.gm_news_vieweds.iter().for_each(|evt| {
        tables
            .create_row("gm_news_viewed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", BigDecimal::from_str(&evt.id).unwrap())
            .set("views", BigDecimal::from_str(&evt.views).unwrap());
    });
    events.gm_ownership_transfer_requesteds.iter().for_each(|evt| {
        tables
            .create_row("gm_ownership_transfer_requested", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.gm_ownership_transferreds.iter().for_each(|evt| {
        tables
            .create_row("gm_ownership_transferred", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("from", Hex(&evt.from).to_string())
            .set("to", Hex(&evt.to).to_string());
    });
    events.gm_request_fulfilleds.iter().for_each(|evt| {
        tables
            .create_row("gm_request_fulfilled", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", Hex(&evt.id).to_string());
    });
    events.gm_request_sents.iter().for_each(|evt| {
        tables
            .create_row("gm_request_sent", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("id", Hex(&evt.id).to_string());
    });
    events.gm_responses.iter().for_each(|evt| {
        tables
            .create_row("gm_response", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("character", &evt.character)
            .set("err", Hex(&evt.err).to_string())
            .set("request_id", Hex(&evt.request_id).to_string())
            .set("response", Hex(&evt.response).to_string());
    });
    events.gm_user_updateds.iter().for_each(|evt| {
        tables
            .create_row("gm_user_updated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
          .set("interest", evt.interest.iter().cloned().collect::<Vec<_>>())
            .set("name", &evt.name)
            .set("profile_url", &evt.profile_url)
            .set("user_address", Hex(&evt.user_address).to_string())
            .set("username", &evt.username);
    });
    events.gm_news_createds.iter().for_each(|evt| {
        tables
            .create_row("gm_news_created", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("blog_id", BigDecimal::from_str(&evt.blog_id).unwrap())
            .set("blog_link", &evt.blog_link)
          .set("categories", evt.categories.iter().cloned().collect::<Vec<_>>());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_gm_events(&blk, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_gm_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_gm_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
