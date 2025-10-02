use std::ffi::CString;
use std::os::raw::c_char;

use redis_module::{
    call_reply_array_element, call_reply_integer, call_reply_length, call_reply_string,
    call_reply_type, free_call_reply, Context, RedisError, RedisModuleCallReply, RedisModuleString,
    RedisModule_Call, RedisResult, RedisString, RedisValue, ReplyType,
};

/// See https://redis.io/docs/reference/modules/modules-api-ref/#redismodule_call
const FMT: *const c_char = c"!vA".as_ptr();

fn parse_call_reply(reply: *mut RedisModuleCallReply) -> RedisResult {
    match call_reply_type(reply) {
        ReplyType::Error => Err(RedisError::String(call_reply_string(reply))),
        ReplyType::Unknown => Err(RedisError::Str("Error on method call")),
        ReplyType::Array => {
            let length = call_reply_length(reply);
            let mut vec = Vec::with_capacity(length);
            for i in 0..length {
                vec.push(parse_call_reply(call_reply_array_element(reply, i))?);
            }
            Ok(RedisValue::Array(vec))
        }
        ReplyType::Integer => Ok(RedisValue::Integer(call_reply_integer(reply))),
        ReplyType::String => Ok(RedisValue::SimpleString(call_reply_string(reply))),
        ReplyType::Null => Ok(RedisValue::Null),
    }
}

pub fn call(ctx: &Context, command: &str, args: &[&RedisString]) -> RedisResult {
    let inner_args: Vec<*mut RedisModuleString> = args.iter().map(|s| s.inner).collect();

    let cmd = CString::new(command).unwrap();
    let reply: *mut RedisModuleCallReply = unsafe {
        let p_call = RedisModule_Call.unwrap();
        p_call(
            ctx.ctx,
            cmd.as_ptr(),
            FMT,
            inner_args.as_ptr() as *mut c_char,
            args.len(),
        )
    };

    let result = parse_call_reply(reply);
    if !reply.is_null() {
        free_call_reply(reply);
    }

    result
}
