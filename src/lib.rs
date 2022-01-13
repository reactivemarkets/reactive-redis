#[macro_use]
extern crate redis_module;

use redis_module::{Context, RedisResult, RedisString};
mod commands;
use commands::{pubsub, streams};

/// Posts a message to the given channel and stores it at the corresponding key
///
/// PUBLISHSET <channel> <message>
fn publishset(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    pubsub::publishset(ctx, args)
}

/// Posts a message to the given channel, stores it at the corresponding key and expires after seconds.
///
/// PUBLISHSETEX <channel> <seconds> <message>
fn publishsetex(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    pubsub::publishsetex(ctx, args)
}

/// Posts a message to the given channel, stores it at the corresponding key and expires after seconds.
///
/// XMREVRANGE end start COUNT count key [key ...]
fn xmrevrange(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    streams::xmrevrange(ctx, args)
}

//////////////////////////////////////////////////////

redis_module! {
    name: "reactive-redis",
    version: 1,
    data_types: [],
    commands: [
        ["publishset", publishset, "write deny-oom pubsub", 0, 0, 0],
        ["publishsetex", publishsetex, "write deny-oom pubsub", 0, 0, 0],
        ["xmrevrange", xmrevrange, "readonly", 0, 0, 0],
    ],
}
