use std::time::Duration;

use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};

use super::redis_module_ext::call;

pub fn mpublish(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let number_of_args = args.len();
    if number_of_args < 3 {
        return Err(RedisError::WrongArity);
    }

    let Some((message, channels)) = args[1..].split_last() else {
        return Err(RedisError::WrongArity);
    };

    let mut response: Vec<RedisValue> = Vec::with_capacity(channels.len());

    for channel in channels {
        let result = call(ctx, "publish", &[channel, message]);
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

pub fn publishset(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 3 {
        return Err(RedisError::WrongArity);
    }

    let [_, channel, message] = &args[..] else {
        return Err(RedisError::WrongArity);
    };

    let redis_key = ctx.open_key_writable(channel);

    let result = redis_key.write(&message.to_string_lossy())?;

    call(ctx, "publish", &[channel, message])?;

    Ok(result)
}

pub fn publishsetex(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 4 {
        return Err(RedisError::WrongArity);
    }

    let [_, channel, seconds, message] = &args[..] else {
        return Err(RedisError::WrongArity);
    };

    let seconds = seconds
        .parse_unsigned_integer()
        .map_err(|_| RedisError::Str("Invalid seconds value"))?;

    let redis_key = ctx.open_key_writable(channel);

    let result = redis_key.write(&message.to_string_lossy())?;
    redis_key.set_expire(Duration::from_secs(seconds))?;

    call(ctx, "publish", &[channel, message])?;

    Ok(result)
}

//////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn run_publishset(args: &[&str]) -> RedisResult {
        let ctx = Context::dummy();

        publishset(&ctx, args.iter().map(|&v| ctx.create_string(v)).collect())
    }

    fn run_mpublish(args: &[&str]) -> RedisResult {
        let ctx = Context::dummy();

        mpublish(&ctx, args.iter().map(|&v| ctx.create_string(v)).collect())
    }

    #[test]
    fn publishset_errors_on_wrong_args() {
        let result = run_publishset(&["PUBLISHSET", "channel", "1", "message"]);

        match result {
            Err(RedisError::WrongArity) => assert!(true),
            _ => assert!(false, "Bad result: {:?}", result),
        }
    }

    #[test]
    fn mpublish_errors_on_wrong_args() {
        // Too few arguments (need at least command + channel + message)
        let result = run_mpublish(&["MPUBLISH", "channel"]);

        match result {
            Err(RedisError::WrongArity) => assert!(true),
            _ => assert!(false, "Bad result: {:?}", result),
        }
    }
}
