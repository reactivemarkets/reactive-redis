use itertools::Itertools;
use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};

use super::redis_module_ext::call;

/// ZUNIONBYSCORE numkeys key [key ...] min max [LIMIT] offset count
pub fn zunionbyscore(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 8 {
        return Err(RedisError::WrongArity);
    }

    let mut args_iter = args.iter().skip(1);
    let numkeys_str = args_iter.next().ok_or(RedisError::WrongArity)?;
    let numkeys = numkeys_str
        .parse_integer()
        .map_err(|_| RedisError::WrongArity)?;

    let keys = &args[2..(2 + numkeys as usize)];
    let min_arg = &args[2 + numkeys as usize];
    let max_arg = &args[3 + numkeys as usize];
    let limit_arg = &args[4 + numkeys as usize];
    let offset_arg = &args[5 + numkeys as usize];
    let count_arg = &args[6 + numkeys as usize];

    if limit_arg.to_string_lossy() != "LIMIT" {
        return Err(RedisError::WrongType);
    }

    let min = min_arg.parse_float().map_err(|_| RedisError::WrongType)?;
    let max = max_arg.parse_float().map_err(|_| RedisError::WrongType)?;

    let offset = offset_arg
        .parse_unsigned_integer()
        .map_err(|_| RedisError::WrongType)?;
    let count = count_arg
        .parse_unsigned_integer()
        .map_err(|_| RedisError::WrongType)?;

    let min_str = min.to_string();
    let max_str = max.to_string();
    let byscore = ctx.create_string("BYSCORE");
    let withscores = ctx.create_string("WITHSCORES");
    let min_redis_str = ctx.create_string(&min_str);
    let max_redis_str = ctx.create_string(&max_str);

    let response = keys
        .iter()
        .filter_map(|key| {
            let results = call(
                ctx,
                "zrange",
                &[key, &min_redis_str, &max_redis_str, &byscore, &withscores],
            )
            .ok()?;
            match results {
                RedisValue::Array(values) => {
                    let keys = values
                        .into_iter()
                        .tuples::<(_, _)>()
                        .collect::<Vec<(_, _)>>();

                    Some(keys)
                }
                _ => None,
            }
        })
        .flatten()
        .map(|tuple| match &tuple.1 {
            RedisValue::SimpleString(score) => {
                let score_f: f64 = score.parse().unwrap_or(0.0);

                (tuple.0, score_f)
            }
            _ => (tuple.0, 0.0),
        })
        .filter(|tuple| tuple.1 >= min && tuple.1 <= max)
        .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .dedup()
        .map(|tuple| tuple.0)
        .skip(offset as usize)
        .take(count as usize)
        .collect::<Vec<_>>();

    Ok(response.into())
}
