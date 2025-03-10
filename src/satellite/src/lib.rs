mod assert;
mod controllers;
mod db;
mod guards;
mod list;
mod msg;
mod rules;
mod storage;
mod types;
mod upgrade;

use crate::controllers::store::get_admin_controllers;
use crate::db::store::{delete_doc, get_doc as get_doc_store, get_docs, insert_doc};
use crate::db::types::interface::{DelDoc, SetDoc};
use crate::db::types::state::{DbHeapState, Doc};
use crate::guards::caller_is_admin_controller;
use crate::rules::constants::DEFAULT_ASSETS_COLLECTIONS;
use crate::rules::store::{
    del_rule_db, del_rule_storage, get_rules_db, get_rules_storage, set_rule_db, set_rule_storage,
};
use crate::rules::types::interface::{DelRule, SetRule};
use crate::rules::types::rules::Rule;
use crate::storage::cert::update_certified_data;
use crate::storage::http::{
    build_encodings, build_headers, create_token, error_response, streaming_strategy,
};
use crate::storage::rewrites::init_rewrites;
use crate::storage::store::{
    commit_batch, create_batch, create_chunk, delete_asset, delete_assets, delete_domain,
    get_config as get_storage_config, get_custom_domains, get_public_asset,
    get_public_asset_for_url, list_assets as list_assets_store, set_config as set_storage_config,
    set_domain,
};
use crate::storage::types::assets::AssetHashes;
use crate::storage::types::config::{StorageConfig, StorageConfigHeaders};
use crate::storage::types::domain::{CustomDomains, DomainName};
use crate::storage::types::http::{
    HttpRequest, HttpResponse, StreamingCallbackHttpResponse, StreamingCallbackToken,
};
use crate::storage::types::http_request::PublicAsset;
use crate::storage::types::interface::{
    AssetNoContent, CommitBatch, InitAssetKey, InitUploadResult, UploadChunk, UploadChunkResult,
};
use crate::storage::types::state::{StorageHeapState, StorageRuntimeState};
use crate::storage::types::store::Asset;
use crate::types::core::CollectionKey;
use crate::types::interface::{Config, RulesType};
use crate::types::list::ListResults;
use crate::types::state::{HeapState, RuntimeState, State};
use crate::upgrade::types::upgrade::UpgradeHeapState;
use controllers::store::{
    delete_controllers as delete_controllers_store, get_controllers,
    set_controllers as set_controllers_store,
};
use ic_cdk::api::call::arg_data;
use ic_cdk::api::{caller, time, trap};
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{export_candid, init, post_upgrade, pre_upgrade, query, update};
use rules::constants::DEFAULT_DB_COLLECTIONS;
use shared::constants::MAX_NUMBER_OF_SATELLITE_CONTROLLERS;
use shared::controllers::{assert_max_number_of_controllers, init_controllers};
use shared::types::interface::{DeleteControllersArgs, SegmentArgs, SetControllersArgs};
use shared::types::state::{ControllerScope, Controllers};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use types::list::ListParams;

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[init]
fn init() {
    let call_arg = arg_data::<(Option<SegmentArgs>,)>().0;
    let SegmentArgs { controllers } = call_arg.unwrap();

    let now = time();

    let db: DbHeapState = DbHeapState {
        db: HashMap::from(
            DEFAULT_DB_COLLECTIONS
                .map(|(collection, _rules)| (collection.to_owned(), BTreeMap::new())),
        ),
        rules: HashMap::from(DEFAULT_DB_COLLECTIONS.map(|(collection, rule)| {
            (
                collection.to_owned(),
                Rule {
                    read: rule.read,
                    write: rule.write,
                    max_size: rule.max_size,
                    created_at: now,
                    updated_at: now,
                },
            )
        })),
    };

    let storage: StorageHeapState = StorageHeapState {
        assets: HashMap::new(),
        rules: HashMap::from(DEFAULT_ASSETS_COLLECTIONS.map(|(collection, rule)| {
            (
                collection.to_owned(),
                Rule {
                    read: rule.read,
                    write: rule.write,
                    max_size: rule.max_size,
                    created_at: now,
                    updated_at: now,
                },
            )
        })),
        config: StorageConfig {
            headers: StorageConfigHeaders::default(),
            rewrites: init_rewrites(),
        },
        custom_domains: HashMap::new(),
    };

    STATE.with(|state| {
        *state.borrow_mut() = State {
            heap: HeapState {
                controllers: init_controllers(&controllers),
                db,
                storage,
            },
            runtime: RuntimeState::default(),
        };
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| stable_save((&state.borrow().heap,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (upgrade_heap,): (UpgradeHeapState,) = stable_restore().unwrap();

    let heap = HeapState::from(&upgrade_heap);

    let asset_hashes = AssetHashes::from(&heap.storage);

    STATE.with(|state| {
        *state.borrow_mut() = State {
            heap,
            runtime: RuntimeState {
                storage: StorageRuntimeState {
                    chunks: HashMap::new(),
                    batches: HashMap::new(),
                    asset_hashes: asset_hashes.clone(),
                },
            },
        }
    });

    update_certified_data(&asset_hashes);
}

///
/// Db
///

#[update]
fn set_doc(collection: CollectionKey, key: String, doc: SetDoc) -> Doc {
    let caller = caller();

    let result = insert_doc(caller, collection, key, doc);

    match result {
        Ok(doc) => doc,
        Err(error) => trap(&error),
    }
}

#[query]
fn get_doc(collection: CollectionKey, key: String) -> Option<Doc> {
    let caller = caller();

    let result = get_doc_store(caller, collection, key);

    match result {
        Ok(value) => value,
        Err(error) => trap(&error),
    }
}

#[update]
fn del_doc(collection: CollectionKey, key: String, doc: DelDoc) {
    let caller = caller();

    delete_doc(caller, collection, key, doc).unwrap_or_else(|e| trap(&e));
}

#[query]
fn list_docs(collection: CollectionKey, filter: ListParams) -> ListResults<Doc> {
    let caller = caller();

    let result = get_docs(caller, collection, &filter);

    match result {
        Ok(value) => value,
        Err(error) => trap(&error),
    }
}

/// Rules

#[query(guard = "caller_is_admin_controller")]
fn list_rules(rules_type: RulesType) -> Vec<(CollectionKey, Rule)> {
    match rules_type {
        RulesType::Db => get_rules_db(),
        RulesType::Storage => get_rules_storage(),
    }
}

#[update(guard = "caller_is_admin_controller")]
fn set_rule(rules_type: RulesType, collection: CollectionKey, rule: SetRule) {
    match rules_type {
        RulesType::Db => set_rule_db(collection, rule).unwrap_or_else(|e| trap(&e)),
        RulesType::Storage => set_rule_storage(collection, rule).unwrap_or_else(|e| trap(&e)),
    }
}

#[update(guard = "caller_is_admin_controller")]
fn del_rule(rules_type: RulesType, collection: CollectionKey, rule: DelRule) {
    match rules_type {
        RulesType::Db => del_rule_db(collection, rule).unwrap_or_else(|e| trap(&e)),
        RulesType::Storage => del_rule_storage(collection, rule).unwrap_or_else(|e| trap(&e)),
    }
}

///
/// Controllers
///

#[update(guard = "caller_is_admin_controller")]
fn set_controllers(
    SetControllersArgs {
        controllers,
        controller,
    }: SetControllersArgs,
) -> Controllers {
    match controller.scope {
        ControllerScope::Write => {}
        ControllerScope::Admin => {
            let max_controllers = assert_max_number_of_controllers(
                &get_admin_controllers(),
                &controllers,
                MAX_NUMBER_OF_SATELLITE_CONTROLLERS,
            );

            if let Err(err) = max_controllers {
                trap(&err)
            }
        }
    }

    set_controllers_store(&controllers, &controller);
    get_controllers()
}

#[update(guard = "caller_is_admin_controller")]
fn del_controllers(DeleteControllersArgs { controllers }: DeleteControllersArgs) -> Controllers {
    delete_controllers_store(&controllers);
    get_controllers()
}

#[query(guard = "caller_is_admin_controller")]
fn list_controllers() -> Controllers {
    get_controllers()
}

///
/// Config
///

#[update(guard = "caller_is_admin_controller")]
fn set_config(config: Config) {
    set_storage_config(&config.storage);
}

#[update(guard = "caller_is_admin_controller")]
fn get_config() -> Config {
    let storage = get_storage_config();
    Config { storage }
}

///
/// Custom domains
///

#[query(guard = "caller_is_admin_controller")]
fn list_custom_domains() -> CustomDomains {
    get_custom_domains()
}

#[update(guard = "caller_is_admin_controller")]
fn set_custom_domain(domain_name: DomainName, bn_id: Option<String>) {
    set_domain(&domain_name, &bn_id);
}

#[update(guard = "caller_is_admin_controller")]
fn del_custom_domain(domain_name: DomainName) {
    delete_domain(&domain_name);
}

///
/// Storage
///

///
/// Http
///

#[query]
fn http_request(
    HttpRequest {
        method,
        url,
        headers: req_headers,
        body: _,
    }: HttpRequest,
) -> HttpResponse {
    if method != "GET" {
        return error_response(405, "Method Not Allowed.".to_string());
    }

    let result = get_public_asset_for_url(url);

    match result {
        Ok(PublicAsset {
            asset,
            url: requested_url,
        }) => match asset {
            Some(asset) => {
                let encodings = build_encodings(req_headers);

                for encoding_type in encodings.iter() {
                    if let Some(encoding) = asset.encodings.get(encoding_type) {
                        let headers =
                            build_headers(&requested_url, &asset, encoding, encoding_type);

                        let Asset {
                            key,
                            headers: _,
                            encodings: _,
                            created_at: _,
                            updated_at: _,
                        } = &asset;

                        match headers {
                            Ok(headers) => {
                                return HttpResponse {
                                    body: encoding.content_chunks[0].clone(),
                                    headers: headers.clone(),
                                    status_code: 200,
                                    streaming_strategy: streaming_strategy(
                                        key,
                                        encoding,
                                        encoding_type,
                                        &headers,
                                    ),
                                }
                            }
                            Err(err) => {
                                return error_response(
                                    405,
                                    ["Permission denied. Invalid headers. ", err].join(""),
                                );
                            }
                        }
                    }
                }

                error_response(500, "No asset encoding found.".to_string())
            }
            None => error_response(404, "No asset found.".to_string()),
        },
        Err(err) => error_response(
            405,
            ["Permission denied. Cannot perform this operation. ", err].join(""),
        ),
    }
}

#[query]
fn http_request_streaming_callback(
    StreamingCallbackToken {
        token,
        headers,
        index,
        sha256: _,
        full_path,
        encoding_type,
    }: StreamingCallbackToken,
) -> StreamingCallbackHttpResponse {
    let asset = get_public_asset(full_path, token);

    match asset {
        Some(asset) => {
            let encoding = asset.encodings.get(&encoding_type);

            match encoding {
                Some(encoding) => StreamingCallbackHttpResponse {
                    token: create_token(&asset.key, index, encoding, &encoding_type, &headers),
                    body: encoding.content_chunks[index].clone(),
                },
                None => trap("Streamed asset encoding not found."),
            }
        }
        None => trap("Streamed asset not found."),
    }
}

//
// Upload
//

#[update]
fn init_asset_upload(init: InitAssetKey) -> InitUploadResult {
    let caller = caller();
    let result = create_batch(caller, init);

    match result {
        Ok(batch_id) => InitUploadResult { batch_id },
        Err(error) => trap(&error),
    }
}

#[update]
fn upload_asset_chunk(chunk: UploadChunk) -> UploadChunkResult {
    let caller = caller();

    let result = create_chunk(caller, chunk);

    match result {
        Ok(chunk_id) => UploadChunkResult { chunk_id },
        Err(error) => trap(error),
    }
}

#[update]
fn commit_asset_upload(commit: CommitBatch) {
    let caller = caller();

    let result = commit_batch(caller, commit);

    match result {
        Ok(_) => (),
        Err(error) => trap(&error),
    }
}

//
// List and delete
//

#[query]
fn list_assets(collection: CollectionKey, filter: ListParams) -> ListResults<AssetNoContent> {
    let caller = caller();

    let result = list_assets_store(caller, collection, &filter);

    match result {
        Ok(result) => result,
        Err(error) => trap(&["Assets cannot be listed: ".to_string(), error].join("")),
    }
}

#[update]
fn del_asset(collection: CollectionKey, full_path: String) {
    let caller = caller();

    let result = delete_asset(caller, collection, full_path);

    match result {
        Ok(_) => (),
        Err(error) => trap(&["Asset cannot be deleted: ", &error].join("")),
    }
}

#[update(guard = "caller_is_admin_controller")]
fn del_assets(collection: CollectionKey) {
    delete_assets(collection);
}

/// Mgmt

#[query]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Generate did files

export_candid!();
