# TenebraStake

A stake node for the "cryptocurrency" [tenebra](https://tenebra.lil.gay).

## Options

| Option       | Description                                                             | Flag | Environment variable  | Default |
|--------------|-------------------------------------------------------------------------|------|-----------------------|---------|
| Private key  | The private-key to connect to tenebra with                              | `-p` | `TENEBRA_PRIVATE_KEY` | None    |
| Reconnecting | Will reconnect if the websocket disconnects. Will late submit if needed | `-r` | `TENEBRA_RECONNECT`   | Enabled |

## How to use

Download the binary, and run it with `TenebraStake -p [myPrivateKey]`.
If you would like to have late-submitting enabled, also add the flag `-l`

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