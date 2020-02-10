#[macro_use]
extern crate redis_module;

use redis_module::{Context, NextArg, RedisError, RedisResult};

/// Posts a message to the given channel and stores it at the corresponding key
///
/// T.PUBLISHSET <channel> <message>
fn publishset(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 3 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let channel = mutable_args.next_string()?;
    let message = mutable_args.next_string()?;

    let redis_key = ctx.open_key_writable(&channel);

    redis_key.write(&message)?;

    let result = ctx.call("publish", &[&channel, &message]).unwrap().into();

    Ok(result)
}

/// Listen for messages published to the given channels
///
/// T.SUBSCRIBEGET <channel> [channel...]
fn subscribeget(ctx: &Context, args: Vec<String>) -> RedisResult {
    let channels: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();

    ctx.call("subscribe", channels.as_slice())?;

    let mut response = Vec::new();

    for channel in &args {
        let redis_key = ctx.open_key(channel);
        match redis_key.read().unwrap() {
            None => {}
            Some(value) => {
                response.push(value);
            }
        }
    }

    Ok(response.into())
}

redis_module! {
    name: "toolbox-redis",
    version: 1,
    data_types: [],
    commands: [
        ["t.publishset", publishset, "write pubsub"],
        ["t.subscribeget", subscribeget, "readonly pubsub"],
    ],
}
