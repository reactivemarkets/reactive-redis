# Toolbox Redis

Custom commands for Redis.

## Commands

### T.PUBLISHSET \<channel> \<message>

Posts a message to the given channel and stores it at the corresponding key.

## Building

```bash
git clone https://github.com/reactivemarkets/toolbox-redis.get
cd toolbox-redis
cargo build --release
```

## Run

### Linux

```bash
redis-server --loadmodule ./target/release/libtoolbox_redis.so
```

### Mac

```bash
redis-server --loadmodule ./target/release/libtoolbox_redis.dylib
```

## Contributing

Refer to [CONTRIBUTING.md](./CONTRIBUTING.md)
