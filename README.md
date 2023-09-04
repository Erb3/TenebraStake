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

## Todo

- [x] Basic functioning
- [ ] Reconnect
- [ ] Wait for keepalive packets
- [ ] Prometheus export?