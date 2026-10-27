# torrentds

DHT torrent-metadata indexer + tracker: bencode wire codec, byte-exact infohash, KRPC (BEP-5).

Part of the [astrx suite](https://github.com/Vxmdzhb/astrx-suite): one of its engines. The suite repository assembles every engine into the single `astrx` binary (`astrx torrentds`) and deploys it.

## Build

    cargo build --release --features net   # the `torrentds` binary
    cargo test --all-features

Features: `rand`, `net`. The default build uses the standard library only; `net` (and `rand`) add the suite's two vetted dependencies, tokio and getrandom. CI asserts the default build stays first-party.

## History

This repository was split out of https://github.com/Vxmdzhb/astrx-suite with the full history of its files, each followed back through its renames. The cross-check goldens are explained in [GOLDENS.md](GOLDENS.md).

## License

Released under the [Do Whatever License](LICENSE), with a code of conduct
in the same spirit: [CODE_OF_CONDUCT](CODE_OF_CONDUCT).
