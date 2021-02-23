#[macro_use]
extern crate redis_module;

use redis_module::{Context, NextArg, RedisError, RedisResult, RedisValue};
use std::time::Duration;

/// Posts a message to the given channel and stores it at the corresponding key
///
/// PUBLISHSET <channel> <message>
fn publishset(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 3 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let channel = mutable_args.next_string()?;
    let message = mutable_args.next_string()?;

    let redis_key = ctx.open_key_writable(&channel);
    redis_key.write(&message)?;

    let result = ctx.call("publish", &[&channel, &message])?;

    Ok(result.into())
}

/// Posts a message to the given channel, stores it at the corresponding key and expires after seconds.
///
/// PUBLISHSETEX <channel> <seconds> <message>
fn publishsetex(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 4 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let channel = mutable_args.next_string()?;
    let seconds = mutable_args.next_u64()?;
    let message = mutable_args.next_string()?;

    let redis_key = ctx.open_key_writable(&channel);

    redis_key.write(&message)?;
    redis_key.set_expire(Duration::from_secs(seconds))?;

    let result = ctx.call("publish", &[&channel, &message])?;

    Ok(result.into())
}

/// Posts a message to the given channel, stores it at the corresponding key and expires after seconds.
///
/// XMREVRANGE end start COUNT count key [key ...]
fn xmrevrange(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() < 6 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let end = mutable_args.next_string()?;
    let start = mutable_args.next_string()?;
    let keyword = mutable_args.next_string()?;
    let count = mutable_args.next_string()?;

    let mut response: Vec<RedisValue> = Vec::new();

    for key in mutable_args {
        let result = ctx.call("xrevrange", &[&key, &end, &start, &keyword, &count]);
        match result {
            Ok(value) => {
                response.push(value)
            },
            Err(_error) => {
                response.push(RedisValue::Null)
            }
        }
    }

    Ok(response.into())
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

//////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn run_publishset(args: &[&str]) -> RedisResult {
        publishset(
            &Context::dummy(),
            args.iter().map(|v| String::from(*v)).collect(),
        )
    }

    #[test]
    fn publishset_errors_on_wrong_args() {
        let result = run_publishset(&vec!["PUBLISHSET", "channel", "1", "message"]);

        match result {
            Err(RedisError::WrongArity) => {
                assert!(true)
            },
            _ => assert!(false, "Bad result: {:?}", result),
        }
    }
}