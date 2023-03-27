# StackBloatLess

Blazingly fast, clean, and effective **native** [Linux](https://en.wikipedia.org/wiki/Linux) desktop GUI for **StackExchange** sites.

## Features

- 🤹 Tabs to open multiple questions
- 🔖 Bookmarks `[TODO]`
- 🔗 Can open URIs in form `stackexchange://{site}/{ids}`, so you can redirect links to it.
- ⚙️ Proxy configurations `[TODO]`
- 🚫 Microsoft Windows is not supported

## Installation

## Build

> **Note**
> You need to have [`cargo`](https://doc.rust-lang.org/cargo/) installed in you system.

```shell
git clone https://github.com/zer0-x/stackbloatless.git

cd stackbloatless

# Checkout to a release tag e.g. v1.0.1
git checkout vx.x.x

cargo build --release
```

You will find the binary in `./target/release/stackbloatless`

## Inspired by

- [AnonymousOverflow](https://github.com/httpjamesm/AnonymousOverflow)
- [so](https://github.com/samtay/so)
