use std::time::Duration;

use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

use super::redis_module_ext::call;

pub fn mpublish(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let number_of_args = args.len();
    if number_of_args < 3 {
        return Err(RedisError::WrongArity);
    }

    let args_len = number_of_args - 1;
    let message = &args[args_len];
    let channels = &args[1..args_len];

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

    let mut mutable_args = args.into_iter().skip(1);
    let channel = mutable_args.next_arg()?;
    let message = mutable_args.next_arg()?;

    let redis_key = ctx.open_key_writable(&channel);

    let result = redis_key.write(&message.to_string_lossy())?;

    call(ctx, "publish", &[&channel, &message])?;

    Ok(result)
}

pub fn publishsetex(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 4 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let channel = mutable_args.next_arg()?;
    let seconds = mutable_args.next_u64()?;
    let message = mutable_args.next_arg()?;

    let redis_key = ctx.open_key_writable(&channel);

    let result = redis_key.write(&message.to_string_lossy())?;
    redis_key.set_expire(Duration::from_secs(seconds))?;

    call(ctx, "publish", &[&channel, &message])?;

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
