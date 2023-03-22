#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisResult, RedisString};
mod commands;
use commands::{pubsub, sorted_sets, streams};

/// Posts a message to one or more channels.
///
/// MPUBLISH channel [channel ...] message
fn mpublish(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    pubsub::mpublish(ctx, args)
}

/// Posts a message to the given channel and stores it at the corresponding key.
///
/// PUBLISHSET <channel> <message>
fn publishset(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    pubsub::publishset(ctx, args)
}

/// Posts a message to the given channel, stores it at the corresponding key and
/// expires after seconds.
///
/// PUBLISHSETEX <channel> <seconds> <message>
fn publishsetex(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    pubsub::publishsetex(ctx, args)
}

/// This command is exactly like XREVRANGE but allows retrieving multiple
/// streams at the same time.
///
/// XMREVRANGE end start COUNT count key [key ...]
fn xmrevrange(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    streams::xmrevrange(ctx, args)
}

/// This command is similar to ZUNION, but reduces the results set by a min and
/// max score.
///
/// ZUNIONBYSCORE numkeys key [key ...] min max [LIMIT] offset count
fn zunionbyscore(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    sorted_sets::zunionbyscore(ctx, args)
}

//////////////////////////////////////////////////////

redis_module! {
    name: "reactive-redis",
    version: 1,
    data_types: [],
    commands: [
        ["mpublish", mpublish, "pubsub", 1, -1, 1],
        ["publishset", publishset, "write deny-oom pubsub", 0, 0, 0],
        ["publishsetex", publishsetex, "write deny-oom pubsub", 0, 0, 0],
        ["zunionbyscore", zunionbyscore, "readonly", 0, 0, 0],
        ["xmrevrange", xmrevrange, "readonly", 0, 0, 0],
    ],
}
