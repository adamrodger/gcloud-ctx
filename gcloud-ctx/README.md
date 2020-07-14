# gcloud-ctx

[![Crate](https://img.shields.io/crates/v/gcloud-ctx.svg)](https://crates.io/crates/gcloud-ctx)
[![API](https://docs.rs/gcloud-ctx/badge.svg)](https://docs.rs/gcloud-ctx)
[![License](https://img.shields.io/github/license/adamrodger/gcloud-ctx)](https://github.com/adamrodger/gcloud-ctx)

A Rust implementation of [`gcloud config configurations`](https://cloud.google.com/sdk/gcloud/reference/config/configurations)
for managing different `gcloud` configurations for Google Cloud Platform. This is the library containing the core logic
which is used to build the associated [`gctx`](https://github.com/adamrodger/gctx) command line utility.

**Note**: `gcloud-ctx` is independent and not affiliated with Google in any way.

## Usage

```rust
use gcloud_ctx::ConfigurationStore;

let mut store = ConfigurationStore::with_default_location()?;

// list configurations
for config in store.configurations() {
    println!("{}", config.name());
}

// get the active configuration
println!("{}", store.active());

// activate a configuration by name
store.activate("foo")?;

// create a new configuration, optionally with a force overwrite
let properties = PropertiesBuilder::default()
    .with_project("my-project")
    .with_account("a.user@example.org")
    .with_zone("europe-west1-d")
    .with_region("europe-west1")
    .build();

store.create("foo", &properties, true)?;

// copy an existing configuration, with force overwrite
store.copy("foo", "bar", true)?;

// rename an existing configuration, with force overwrite
store.rename("foo", "bar", true)?;

// delete a configuration
store.delete("foo")?;

// get properties of a configuration
let properties = store.describe(name)?;
properties.to_writer(std::io::stdout())
```

## License

`gcloud-ctx` is distributed under the terms of the MIT license
