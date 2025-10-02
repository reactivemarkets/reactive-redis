use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};

use super::redis_module_ext::call;

/// XMREVRANGE end start COUNT count key [key ...]
pub fn xmrevrange(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 6 {
        return Err(RedisError::WrongArity);
    }

    let [_, end, start, keyword, count, keys @ ..] = &args[..] else {
        return Err(RedisError::WrongArity);
    };

    let mut response: Vec<RedisValue> = Vec::with_capacity(keys.len());

    for key in keys {
        let result = call(ctx, "xrevrange", &[key, end, start, keyword, count]);
        match result {
            Ok(value) => {
                response.push(value);
            }
            Err(_error) => {
                response.push(RedisValue::Null);
            }
        }
    }

    Ok(response.into())
}
