# TenebraStake

> [!IMPORTANT]
> TenebraStake is archived as of August 20th, and will not receive any updates.

[![github](https://cdn.jsdelivr.net/npm/@intergrav/devins-badges@3/assets/cozy/available/github_vector.svg)](https://github.com/Erb3/tenebrastake)
![risugamis-modloader](https://cdn.jsdelivr.net/npm/@intergrav/devins-badges@3/assets/cozy/unsupported/risugamis-modloader_vector.svg)

A stake node for the "cryptocurrency" [tenebra](https://tenebra.lil.gay).

## Options

| Option      | Description                                                                     | Flag | Environment variable | Default                   |
| ----------- | ------------------------------------------------------------------------------- | ---- | -------------------- | ------------------------- |
| Private key | The private-key to connect to tenebra with                                      | `-p` | `PRIVATE_KEY`        | None                      |
| Sync node   | The tenebra node to connect to. You will in most cases not need to change this. | `-s` | `SYNC_NODE`          | `https://tenebra.lil.gay` |

## How to use

Download the binary, and run it with `TenebraStake -p [myPrivateKey]`.

### Docker run

Run the following command to start the stake node with `docker run`. Replace `abc123` with your private key:

```shell
docker run -d -e PRIVATE_KEY=abc123 ghcr.io/erb3/tenebrastake:latest
```

### Docker compose

To run with Docker compose, add this to your `docker-compose.yml`. Replace `abc123` with your private key:

```yml
tenebrastake:
 image: ghcr.io/erb3/tenebrastake:latest
 container_name: tenebrastake
 restart: unless-stopped
 environment:
  - PRIVATE_KEY=abc123
```

## Requirements

The websocket and HTTPS parts of TenebraStake use native TLS. If you are doing something else than running the binary on Linux, you don't need anything!

### Linux

OpenSSL 1.0.1, 1.0.2, 1.1.0, or 1.1.1 with headers (see [rust-openssl](https://github.com/sfackler/rust-openssl)).

## Versioning

TenebraStake is versioned with my own proprietary versioning scheme known as `rewrite.major.minor`.
In major updates, it may be breaking. It rewrites, it will be breaking. Patch releases will not be intentionally breaking!

## FAQ

**Can I run multiple stake nodes with the same wallet?**

Answer: Yes, you can run multiple stake nodes.
However, the only benefit to this is that one will take over if one crashes (which should not happen).
You do not get any financial gain.

**Does it automatically reconnect?**

TenebraStake may in some cases reconnect, such as if you lost internet connectivity for a small amount of time.
However, TenebraStake is meant to be run on servers, so it is meant to crash when weird things happen.
If you want it to restart once it crashed, consider using docker with restarting enabled.

## Todo

- [ ] Wait for keep-alive packets
- [ ] Prometheus export
- [ ] Remove all stakes on exit (and add all stakes on start)
- [ ] Remove all stakes if the total network stake is above a set threshold
- [ ] Detect stake getting disabled

## Alternatives

Don't like TenebraStake? 😥. If it was our fault, please make a GitHub issue! Alternatives we like:

1. [TenebraStakeNode](https://github.com/PatriikPlays/tenebrastakenode/) by PatriikPlays
2. [TenebraValidator](https://github.com/xAnavrins/TenebraValidator) by Anavrins
3. [tenebra.lua](https://gist.github.com/Ale32bit/2978fd3962506a8a943fbcf115084b6b) by AlexDevs
4. [TenebraStakingNode](https://github.com/Allymonies/TenebraStakingNode) by Allymonies
