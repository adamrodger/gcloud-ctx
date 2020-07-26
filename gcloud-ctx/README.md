# gcloud-ctx

[![Crate](https://img.shields.io/crates/v/gcloud-ctx.svg)](https://crates.io/crates/gcloud-ctx)
[![API](https://docs.rs/gcloud-ctx/badge.svg)](https://docs.rs/gcloud-ctx)
[![License](https://img.shields.io/github/license/adamrodger/gcloud-ctx)](https://github.com/adamrodger/gcloud-ctx)

<!-- cargo-sync-readme start -->

A Rust implementation of [`gcloud config configurations`](https://cloud.google.com/sdk/gcloud/reference/config/configurations)
for managing different `gcloud` configurations for Google Cloud Platform. This is the library containing the core logic
which is used to build the associated [`gctx`](https://github.com/adamrodger/gctx) command line utility.

**Note**: `gcloud-ctx` is independent and not affiliated with Google in any way.

## Usage

```rust
use gcloud_ctx::{ConfigurationStore, ConflictAction};

let mut store = ConfigurationStore::with_default_location()?;

// create a new configuration, optionally with a force overwrite
use gcloud_ctx::PropertiesBuilder;
let properties = PropertiesBuilder::default()
    .project("my-project")
    .account("a.user@example.org")
    .zone("europe-west1-d".parse()?)
    .region("europe-west1".parse()?)
    .build();

store.create("foo", &properties, ConflictAction::Overwrite)?;

// list configurations
for config in store.configurations() {
    println!("{}", config.name());
}

// activate a configuration by name
store.activate("foo")?;

// get the active configuration
println!("{}", store.active());

// copy an existing configuration, with force overwrite
store.copy("foo", "bar", ConflictAction::Overwrite)?;

// rename an existing configuration, with force overwrite
store.rename("bar", "baz", ConflictAction::Overwrite)?;

// delete a configuration
store.delete("baz")?;

// get properties of a configuration
let properties = store.describe("foo")?;
properties.to_writer(std::io::stdout())?;
```

<!-- cargo-sync-readme end -->

## License

`gcloud-ctx` is distributed under the terms of the MIT license
