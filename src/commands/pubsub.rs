use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString};
use std::time::Duration;

pub fn publishset(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 3 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let channel = mutable_args.next_arg()?;
    let message = mutable_args.next_string()?;

    let redis_key = ctx.open_key_writable(&channel);

    let result = redis_key.write(&message)?.into();

    let str_channel = RedisString::to_string_lossy(&channel);

    ctx.call("publish", &[&str_channel, &message])?;

    Ok(result)
}

pub fn publishsetex(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 4 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let channel = mutable_args.next_arg()?;
    let seconds = mutable_args.next_u64()?;
    let message = mutable_args.next_string()?;

    let redis_key = ctx.open_key_writable(&channel);

    let result = redis_key.write(&message)?.into();
    redis_key.set_expire(Duration::from_secs(seconds))?;

    let str_channel = RedisString::to_string_lossy(&channel);

    ctx.call("publish", &[&str_channel, &message])?;

    Ok(result)
}

//////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn run_publishset(args: &[&str]) -> RedisResult {

        let ctx = Context::dummy();

        publishset(
            &ctx,
            args.iter().map(|&v| ctx.create_string(v)).collect(),
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
