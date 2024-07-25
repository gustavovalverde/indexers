use alloy::{eips::BlockId, sol};
use ghost_crab::prelude::*;

use crate::db;

sol!(
    #[sol(rpc)]
    StaderStakePoolsManager,
    "abis/stader/StaderStakePoolsManager.json"
);

const STADER: Address = address!("cf5EA1b38380f6aF39068375516Daf40Ed70D299");

#[block_handler(Stader)]
async fn StaderBlockHandler(ctx: BlockContext) {
    let stader_staking_manager = StaderStakePoolsManager::new(STADER, &ctx.provider);

    let total_assets = stader_staking_manager
        .totalAssets()
        .block(BlockId::from(ctx.block_number))
        .call()
        .await
        .unwrap();

    let db = db::get().await;

    let eth = total_assets._0.to_string();
    let block_number = ctx.block_number as i64;
    let block = ctx.block().await.unwrap().unwrap();
    let block_timestamp = block.header.timestamp as i64;

    sqlx::query!(
        r#"insert into "Stader" (block_number, block_timestamp, eth) values ($1,$2,$3)"#,
        block_number,
        block_timestamp,
        eth,
    )
    .execute(db)
    .await
    .unwrap();
}
