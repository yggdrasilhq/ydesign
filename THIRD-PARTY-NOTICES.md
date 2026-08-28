# Third-party notices

ydesign is licensed GPL-3.0-or-later. It links the Rust crates below, all under
permissive licences compatible with that. Exact resolved versions are pinned in
`Cargo.lock`.

| Crate | Licence |
|---|---|
| anyhow | MIT OR Apache-2.0 |
| base64 | MIT OR Apache-2.0 |
| clap | MIT OR Apache-2.0 |
| ctrlc | MIT OR Apache-2.0 |
| dirs | MIT OR Apache-2.0 |
| serde | MIT OR Apache-2.0 |
| serde_json | MIT OR Apache-2.0 |

## Relationship to libyggterm and yggterm

ydesign is a *consumer* of libyggterm, not a linker against it: it speaks the
yggterm control protocol (OSC 7717 plus a loopback HTTP control endpoint) and
runs as its own process. No libyggterm code is compiled into this binary, so
libyggterm's MPL-2.0 terms do not reach it.

The design language the notebooks document — `yggui`, `yggui-contract` and
`emd-renderer` — lives in libyggterm (MPL-2.0) and is painted by the yggterm
host (GPL-3.0-or-later). ydesign authors markdown and widget *schemas*; it
ships none of that code.
