# Reactive Redis

Custom commands for Redis. We don't prefix these commands with anything as they currently don't clash.

## Commands

### `PUBLISHSET channel message`

Posts a message to the given channel and stores it at the corresponding key.

### `PUBLISHSETEX channel seconds message`

Posts a message to the given channel, stores it at the corresponding key and sets an expiry in seconds.

### `XMREVRANGE end start COUNT count key [key ...]`

This command is exactly like XREVRANGE but allows retrieving multiple streams at the same time.

### `ZUNIONBYSCORE numkeys key [key ...] min max [LIMIT] offset count`

Similar to ZUNION but only returns results within the given min and max. If LIMIT is specified it will return the count only.

## Building

```bash
git clone https://github.com/reactivemarkets/reactive-redis.get
cd reactive-redis
cargo build
```

## Tests

```
cargo test --features test
```

## Run

### Linux

```bash
redis-server --loadmodule ./target/debug/libreactive_redis.so
```

### Mac

```bash
redis-server --loadmodule ./target/debug/libreactive_redis.dylib
```

## Contributing

Refer to [CONTRIBUTING.md](./CONTRIBUTING.md)
