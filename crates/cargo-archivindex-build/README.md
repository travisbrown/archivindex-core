# cargo-archivindex-build

`cargo-archivindex-build` keeps Cargo workspace policy and formatter configuration consistent
across Archivindex projects.

Install it from a clone of this repository, then run it from any workspace:

```console
cargo install --locked --path crates/cargo-archivindex-build
cargo archivindex-build check
cargo archivindex-build sync
```

Both commands accept `--manifest-path <PATH>`. `check` reports every policy mismatch and exits
unsuccessfully when it finds one. `sync` applies fixes that can be determined mechanically, then
runs the same checks. Values that are specific to a project, such as its repository URL, must
already be present in `[workspace.package]`.

The enforced policy includes the following requirements:

- Cargo resolver version 3;
- shared workspace package metadata (`authors`, `repository`, `edition`, `rust-version`,
  `readme`, `license`, and `version`);
- the Archivindex Rust and Clippy workspace lint configuration;
- workspace lint and package metadata inheritance in non-root member packages;
- a sorted `[workspace.dependencies]` table, with no entry that no member uses, and no member
  restating a dependency the table already declares;
- `description`, `readme`, and docs.rs metadata on packages that can reach a registry;
- the shared `rustfmt.toml` and `.taplo.toml` settings;
- the `deny.toml` settings that decide how strict a `cargo deny` run is.

It deliberately checks nothing that `rustfmt`, `clippy`, `taplo`, or `cargo deny` already
checks (what it adds is the configuration that those tools are run with).

## Exemptions

A rule that some package legitimately cannot follow is waived in the root manifest:

```toml
[[workspace.metadata.archivindex-build.exemptions]]
package = "archivindex-surt"
rule = "dependencies.serde"
reason = "The shared declaration enables `derive`, which this crate implements by hand."
```

Only three rules can be named, because only these have turned out to have legitimate exceptions:

| Rule | Waives |
| --- | --- |
| `package.authors`, `package.license` | A crate that carries its own authorship or license, such as one forked from someone else's. |
| `lints.workspace` | A package that must relax one workspace lint, such as `unsafe_code`. |
| `dependencies.<name>` | A dependency that must be configured per package. |

Every entry must give a `reason`, `sync` leaves an exempted package alone instead of repairing
it, and `check` reports an entry that no longer waives anything, so the list cannot outlive the
situations that justified it.

## License

Licensed under either the [MIT License][mit] or the [Apache License, Version 2.0][apache-2.0], at
your option; see [LICENSE-MIT][license-mit] and [LICENSE-APACHE][license-apache] for the full
texts.

[apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[license-apache]: LICENSE-APACHE
[license-mit]: LICENSE-MIT
[mit]: https://opensource.org/license/mit
