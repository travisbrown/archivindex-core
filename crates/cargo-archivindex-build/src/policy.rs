/// Fields `[workspace.package]` must define so that members have something to inherit.
pub const WORKSPACE_PACKAGE_FIELDS: [&str; 7] = [
    "authors",
    "repository",
    "edition",
    "rust-version",
    "readme",
    "license",
    "version",
];

/// Fields every member package must inherit from `[workspace.package]`.
///
/// `readme` is absent because a crate with its own README file states a path of its own; the
/// requirement on members is only that some readme is named, which [`PUBLISHED_FIELDS`] covers.
pub const INHERITED_PACKAGE_FIELDS: [&str; 6] = [
    "authors",
    "repository",
    "edition",
    "rust-version",
    "license",
    "version",
];

/// Package fields that must be present on a crate that can reach crates.io.
///
/// These are the fields a registry and its readers see and a path dependency never needs, so they
/// are not required of the support crates and build tools that are marked `publish = false`.
pub const PUBLISHED_FIELDS: [&str; 2] = ["description", "readme"];

/// The `rustdoc` arguments that let a published crate document its feature-gated items.
pub const DOCS_RS_RUSTDOC_ARGS: [&str; 2] = ["--cfg", "docsrs"];

/// The docs.rs setting that documents every feature of a published crate.
pub const DOCS_RS_ALL_FEATURES: [&str; 5] = ["package", "metadata", "docs", "rs", "all-features"];

/// The docs.rs setting that carries [`DOCS_RS_RUSTDOC_ARGS`].
pub const DOCS_RS_ARGUMENTS: [&str; 5] = ["package", "metadata", "docs", "rs", "rustdoc-args"];

pub const STRING_LINTS: [(&[&str], &str); 7] = [
    (&["workspace", "lints", "rust", "missing_docs"], "deny"),
    (
        &["workspace", "lints", "rust", "rust_2018_idioms", "level"],
        "warn",
    ),
    (&["workspace", "lints", "rust", "unsafe_code"], "forbid"),
    (&["workspace", "lints", "clippy", "all", "level"], "warn"),
    (
        &["workspace", "lints", "clippy", "pedantic", "level"],
        "warn",
    ),
    (
        &["workspace", "lints", "clippy", "nursery", "level"],
        "warn",
    ),
    (
        &["workspace", "lints", "clippy", "missing_errors_doc"],
        "allow",
    ),
];

pub const PRIORITY_LINTS: [&[&str]; 4] = [
    &["workspace", "lints", "rust", "rust_2018_idioms", "priority"],
    &["workspace", "lints", "clippy", "all", "priority"],
    &["workspace", "lints", "clippy", "pedantic", "priority"],
    &["workspace", "lints", "clippy", "nursery", "priority"],
];

/// Manifest sections that can name a dependency.
pub const DEPENDENCY_SECTIONS: [&str; 3] =
    ["dependencies", "dev-dependencies", "build-dependencies"];

/// `deny.toml` settings that decide how much `cargo deny` checks, rather than what it allows.
///
/// The allowed licenses, the source allowances, and the advisory ignores differ from project to
/// project and are left alone; these settings are what make the run strict enough to be worth
/// having, so they are the same everywhere.
pub const DENY_STRING_SETTINGS: [(&[&str], &str); 4] = [
    (&["bans", "multiple-versions"], "warn"),
    (&["bans", "wildcards"], "deny"),
    (&["sources", "unknown-registry"], "deny"),
    (&["sources", "unknown-git"], "deny"),
];

/// `deny.toml` configuration versions, which select the current semantics of each section.
pub const DENY_INTEGER_SETTINGS: [(&[&str], i64); 2] = [
    (&["advisories", "version"], 2),
    (&["licenses", "version"], 2),
];

/// The `deny.toml` setting that makes the checks cover optional dependencies.
pub const DENY_ALL_FEATURES: [&str; 2] = ["graph", "all-features"];
