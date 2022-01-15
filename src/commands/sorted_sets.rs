use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};
use itertools::Itertools;

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

    keys.push("AGGREGATE");
    keys.push("MAX");
    keys.push("WITHSCORES");

    let min = mutable_args.next_f64()?;
    let max = mutable_args.next_f64()?;

    let limit = mutable_args.next_str()?;
    if limit != "LIMIT" {
        return Err(RedisError::WrongArity);
    }
    
    let offset = mutable_args.next_u64()?;
    let count = mutable_args.next_u64()?;

    let all_keys = ctx.call("zunion", &keys)?;
    match all_keys {
        RedisValue::Array (results) => {
            let response = results
                .into_iter()
                .tuples::<(_, _)>()
                .filter(|tuple| {
                    match &tuple.1 {
                        RedisValue::SimpleString(score) => {
                            if score == "-inf" || score == "+inf" {
                                return true
                            }
        
                            let score_f: f64 = score.parse().unwrap_or(0.0);

                            return score_f > min && score_f < max;
                        }
                        _ => false
                    }
                })
                .map(|tuple| tuple.0)
                .skip(offset as usize)
                .take(count as usize)
                .collect::<Vec<_>>();
            
            return Ok(response.into());
        },
        _ => {
            return Err(RedisError::WrongType);
        }
    }
}
