# TenebraStake

A stake node for the "cryptocurrency" [tenebra](https://tenebra.lil.gay).

## Options

| Option       | Description                                                                     | Flag | Environment variable | Default                   |
|--------------|---------------------------------------------------------------------------------|------|----------------------|---------------------------|
| Private key  | The private-key to connect to tenebra with                                      | `-p` | `PRIVATE_KEY`        | None                      |
| Sync node    | The tenebra node to connect to. You will in most cases not need to change this. | `-s` | `SYNC_NODE`          | `https://tenebra.lil.gay` |

## How to use

Download the binary, and run it with `TenebraStake -p [myPrivateKey]`.

### Docker

Docker images are automatically published to GitHub Packages.

## Requirements

The websocket and HTTPS parts of TenebraStake use native TLS. If you are doing something else than running the binary on Linux, you don't need anything!

### Linux

OpenSSL 1.0.1, 1.0.2, 1.1.0, or 1.1.1 with headers

## Technical limitations

We use unsigned 32-bit integers, which means that you cannot submit more than 4 294 967 294 blocks.
Say you submit a block every minute, then you can run for about 8166 years before reaching the integer limit.
If you plan to run your tenebra staking node for more than 8166 years, without restart, consider checking out [alternatives](#alternatives).

## Versioning

TenebraStake is versioned with my own proprietary versioning scheme known as `rewrite.major.minor`.
In major updates, it may be breaking. It rewrites, it will be breaking. Patch releases will not be intentionally breaking!

## FAQ

**Can I run multiple stake nodes on the same wallet?**

Answer: Yes, you can run multiple stake nodes. However, the only benefit to this, is that one will take over if one crashes (which should not happen).
You do not get any financial gain.

**Does it automatically reconnect?**

TenebraStake does may in some cases reconnect, such as if you lost internet connectivity for a small amount of time.
TenebraStake is meant to be run on servers, so it may not always crash if you loose internet connectivity.

## Todo

- [ ] Wait for keepalive packets
- [ ] Prometheus export?
- [ ] Remove all stake on exit ( and add all stake on start )
- [ ] Remove all stake if total network stake is above threshold
- [ ] Detect stake getting disabled

## Alternatives

Don't like TenebraStake? 😥. If it was our fault, please make a GitHub issue! Alternatives we like:

* [TenebraStakeNode](https://github.com/PatriikPlays/tenebrastakenode/) by PatriikPlays.
* [TenebraStakingNode](https://github.com/Allymonies/TenebraStakingNode) by Allymonies
