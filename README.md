# Reactive Redis

Custom commands for Redis.

## Commands

### R.PUBLISHSET \<channel> \<message>

Posts a message to the given channel and stores it at the corresponding key.

## Building

```bash
git clone https://github.com/reactivemarkets/reactive-redis.get
cd reactive-redis
cargo build --release
```

## Run

### Linux

```bash
redis-server --loadmodule ./target/release/libreactive_redis.so
```

### Mac

```bash
redis-server --loadmodule ./target/release/libreactive_redis.dylib
```

## Contributing

Refer to [CONTRIBUTING.md](./CONTRIBUTING.md)
