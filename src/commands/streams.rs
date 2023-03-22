use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

/// XMREVRANGE end start COUNT count key [key ...]
pub fn xmrevrange(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 6 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let end = mutable_args.next_string()?;
    let start = mutable_args.next_string()?;
    let keyword = mutable_args.next_string()?;
    let count = mutable_args.next_string()?;

    let mut response: Vec<RedisValue> = Vec::new();

    for arg in mutable_args {
        let key = RedisString::to_string_lossy(&arg);
        let result = ctx.call("xrevrange", &[&key, &end, &start, &keyword, &count]);
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
