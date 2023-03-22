use itertools::Itertools;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

/// ZUNIONBYSCORE numkeys key [key ...] min max [LIMIT] offset count
pub fn zunionbyscore(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 8 {
        return Err(RedisError::WrongArity);
    }

    let mut mutable_args = args.into_iter().skip(1);
    let numkeys = mutable_args.next_i64()?;
    let numkeys_string = numkeys.to_string();

    let mut keys: Vec<&str> = Vec::new();
    keys.push(&numkeys_string);

    for _i in 0..numkeys {
        let key = mutable_args.next_str()?;
        keys.push(key);
    }

    let min = mutable_args.next_f64()?;
    let max = mutable_args.next_f64()?;

    let limit = mutable_args.next_str()?;
    if limit != "LIMIT" {
        return Err(RedisError::WrongArity);
    }

    let offset = mutable_args.next_u64()?;
    let count = mutable_args.next_u64()?;

    let response = keys
        .into_iter()
        .filter_map(|key| {
            let min_str = min.to_string();
            let max_str = max.to_string();

            let results = ctx
                .call(
                    "zrange",
                    &[key, &min_str, &max_str, "BYSCORE", "WITHSCORES"],
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
