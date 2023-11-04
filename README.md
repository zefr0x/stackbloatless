<div align = center>

<h1>StackBloatLess</h1>

[![release](https://github.com/zefr0x/stackbloatless/actions/workflows/release.yml/badge.svg)](https://github.com/zefr0x/stackbloatless/actions/workflows/release.yml)

Blazingly fast, clean, and effective **native** [Linux](https://en.wikipedia.org/wiki/Linux) desktop [GUI](https://en.wikipedia.org/wiki/Graphical_user_interface) for [**StackExchange**](https://en.wikipedia.org/wiki/Stack_Exchange_Network) sites.

---

[<kbd><br><b>Install</b><br><br></kbd>](#installation)
[<kbd><br><b>Screenshots</b><br><br></kbd>](#screenshots)
[<kbd><br><b>Contribute</b><br><br></kbd>](CONTRIBUTING.md)
[<kbd><br><b>Packaging</b><br><br></kbd>](PACKAGING.md)

---

<br>

</div>

## Features

- 📜 Clean questions, answers, and comments without any distractions. `[WIP]`
- 🤹 Tabs to open multiple questions.
- 🔖 Bookmarks. `[TODO]`
- 🔗 Can open URIs, so you can redirect StackExchange links to it.
- ⚙️ Proxy configurations `[TODO]`
- 🔎 Simple search engine support. `[TODO]`
- 🚫 Microsoft Windows is not supported.

## Requirements

- [GTK4](https://www.gtk.org/)
- [Adwaita](https://gitlab.gnome.org/GNOME/libadwaita/)

## Installation

### Download Binary From Github

For every new release a Github workflow will build a binary in Github servers and will upload it as a release asset in Github releases.

You can find the latest Github release [here](https://github.com/zefr0x/stackbloatless/releases/latest) or the releases page [here](https://github.com/zefr0x/stackbloatless/releases).

## Build

> **Note**
> You need to have [`cargo`](https://doc.rust-lang.org/cargo/) installed in you system.

```shell
git clone https://github.com/zefr0x/stackbloatless.git

cd stackbloatless

# Checkout to a release tag e.g. v1.0.1
git checkout vx.x.x

cargo build --release
```

You will find the binary in `./target/release/stackbloatless`

## How to use it?

You are able to search for questions from the application using StackExchange's search API, but it's very primitive, so you might not find what you are searching for.

You are recomended to use a web browser and your search engine of choice along with a browser extension to redirect any questions under the StackExchange network to be opened inside StackBloatLess.

StackBloatLess accept StackExchange questions in the next format to be opened in it:

```
stackbloatless://{api_site_parameter}/{ids}
```

Where `{api_site_parameter}` is specific to single StackExchange site that could be found [here](https://api.stackexchange.com/docs/sites#pagesize=500&filter=!SldCuNUOz*uwhNyRzh&run=true), and `{ids}` is a list of questions ids seprated by `;`, like `id;id;id;id...`.

## Screenshots

<!-- TODO: Add images. -->

## Inspired by

- [AnonymousOverflow](https://github.com/httpjamesm/AnonymousOverflow)
- [so](https://github.com/samtay/so)
