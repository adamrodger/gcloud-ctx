# `gctx` - A `gcloud` configuration utility

`gctx`:
![Build and Test](https://github.com/adamrodger/gcloud-ctx/workflows/Build%20and%20Test/badge.svg)
![Version](https://img.shields.io/github/v/tag/adamrodger/gcloud-ctx)
`gcloud-ctx`:
[![Crate](https://img.shields.io/crates/v/gcloud-ctx.svg)](https://crates.io/crates/gcloud-ctx)
[![API](https://docs.rs/gcloud-ctx/badge.svg)](https://docs.rs/gcloud-ctx)

Manage [Google Cloud Platform](https://cloud.google.com/) `gcloud` configurations easily and quickly

## Motivation

I'm often working with multiple GCP projects with a variety of different settings (e.g. default compute zone).
To do this, I take advantage of the [`gcloud config configurations`](https://cloud.google.com/sdk/gcloud/reference/config/configurations)
command in the standard `gcloud` tooling. However, this has two major problems:

- It's quite a lot of typing if you're switching often. Sure, an alias can help, but...
- `gcloud` is slow to initialise which makes it annoying to use and to add to your default prompt

After having used [`kubectx`](https://github.com/ahmetb/kubectx) to solve a similar problem switching between
Kubernetes contexts easily, I searched for a similar tool that could easily switch between `gcloud` configurations
but couldn't find anything. So, having spent a while learning [Rust](https://www.rust-lang.org/) and being incredibly
impressed, I thought it would make a great starter project.

## Goals

`gctx` aims to achieve similar goals to `kubectx`, such as:

- Extremely fast switching between different `gcloud` configurations
- Cross platform
- Shorter to type than `gcloud config configurations activate` 😄
- Fuzzy finding using `fzf` (if installed)

## Installation

### From source

Get the latest stable version of Rust via [`rustup`](https://rustup.rs/) and run:

```bash
cargo install --path .
# gctx will now be on your PATH
```

### Pre-Built Binaries

Grab the [latest binary](https://github.com/adamrodger/gcloud-ctx/releases/latest), extract it and add it to your `PATH`.

### Others (e.g. `scoop` and `brew`)

**TODO**: [Support for `scoop`](https://github.com/adamrodger/gcloud-ctx/issues/13)

**TODO**: [Support for `brew`](https://github.com/adamrodger/gcloud-ctx/issues/14)

## Usage

```bash
# show the current configuration (useful for adding to default prompt)
gctx current
gctx          # shorthand, just omit current

# list all configurations
gctx list

# activate a different configuration
gctx my-config
gctx activate my-config   # explicitly activate, e.g. if your configuration name clashes with a gctx command
gctx activate             # if fzf is installed, you can omit the name and select from a list

# create (and optionally activate) a new configuration
gctx create my-config --project foo \
                      --account a.user@example.org \
                      --zone europe-west1-d \
                      --region europe-west1 \
                      --force \
                      --activate

# copy an existing configuration
gctx copy src-name dest-name --force --activate

# show the properties of a configuration (like gcloud config configurations describe)
gctx describe           # defaults to the current configuration
gctx describe name      # describe a named configuration

# rename a configuration
gctx rename old-name new-name
gctx rename --force old-name existing-name   # use force to overwrite an existing configuration

# delete a configuration. note: you can't delete the active configuration
gctx delete my-config

# show help and usage
gctx --help
```

**Note**: This project is independent and not afilliated with Google in any way
