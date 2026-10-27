# LisekMovie-plugin-audiodb

TheAudioDB metadata provider extension for [LisekMovie](https://github.com/LisekMovieApp/LisekMovie-app), the self-hosted video-on-demand server.

| | |
|---|---|
| Provider key | `audiodb` |
| Covers | Music (artists, albums) |
| Status | Skeleton — registered, returns no results yet |
| API key | Optional — without one the public test key `2` is used |
| Fix match ids | `audiodb:` ids are reserved for artists/albums |

## What it does

Registered skeleton for the coming audio libraries. Film and series questions return nothing without any request.

**TODO:** artist and album search, lookup and artwork once `lisek-plugin-api` has audio item kinds.

## How it plugs in

The crate implements `lisek_plugin_api::MetadataProvider` and is compiled into `lisek-server` as a Cargo path dependency — there is no runtime plugin loading. Clone it **next to** `LisekMovie-app`:

```
LisekMovie/
  LisekMovie-app/
  LisekMovie-plugin-audiodb/
  ...
```

The server lists the provider under **Admin → Plugins** (disabled by default). Turn it on, add the key if it needs one, and set its priority — globally, or per library (for example AniList or aniDB first in an anime library). `install-linux.sh` and `update-linux.sh` clone and update it with the other modules.

## Development

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```

Tests run against a local `wiremock` server; nothing calls the real service.

## License

[MIT](LICENSE), like LisekMovie itself.
