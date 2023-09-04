# TenebraStake

A stake node for the "cryptocurrency" [tenebra](https://tenebra.lil.gay).

## Options

| Option       | Description                                                             | Flag | Environment variable | Default                   |
|--------------|-------------------------------------------------------------------------|------|----------------------|---------------------------|
| Private key  | The private-key to connect to tenebra with                              | `-p` | `PRIVATE_KEY`        | None                      |
| Reconnecting | Will reconnect if the websocket disconnects. Will late submit if needed | `-r` | `RECONNECT`          | Disabled                  |
| Reconnecting | Will reconnect if the websocket disconnects. Will late submit if needed | `-s` | `SYNC_NODE`          | `https://tenebra.lil.gay` |

## How to use

Download the binary, and run it with `TenebraStake -p [myPrivateKey]`.

## Requirements

### Linux

OpenSSL 1.0.1, 1.0.2, 1.1.0, or 1.1.1 with headers

### Windows

Nothing!

### Mac

Nothing!

## Technical limitations

We use unsigned 32-bit integers, which means that you cannot submit more than 4 294 967 294 blocks.
Say you submit a block every minute, then you can run for about 8166 years before reaching the integer limit.
If you plan to run your tenebra staking node for more than 8166 years, without restart, consider using [TenebraStakeNode](https://github.com/PatriikPlays/tenebrastakenode/) by PatriikPlays.

## Todo

- [x] Basic functioning
- [ ] Reconnect
- [ ] Wait for keepalive packets
- [ ] Prometheus export?