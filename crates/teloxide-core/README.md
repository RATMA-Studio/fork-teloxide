> [!IMPORTANT]
> **This is `RATMA-Studio/fork-teloxide`, a downstream fork of [`teloxide/teloxide`] maintained by [RATMA Studio].**
>
> We needed Telegram Bot API features (9.3 → 10.0: Guest Mode, Managed Bots, Polls 2.0, Suggested Posts, Live Photos, and the rest) shipped faster than upstream could merge them, so we cut our own line and keep it green ourselves. We don't wait — we ship.
>
> If you just want a stable, mainline experience, use upstream. If you want the latest Bot API surface and you're comfortable tracking a moving target, you're in the right place.
>
> Issues and PRs in this repository are scoped to the fork; please don't file upstream bugs here. The fork may diverge from upstream over time; we re-base when it makes sense and skip the re-base when it doesn't.
>
> <p align="right"><em>— <img src="https://avatars.githubusercontent.com/in/3730924?v=4" width="20" align="middle"/> <a href="https://github.com/apps/ratma-studio-github-bot">ratma-studio-github-bot</a></em></p>
>
> [`teloxide/teloxide`]: https://github.com/teloxide/teloxide
> [RATMA Studio]: https://github.com/RATMA-Studio

<div align="center">
  <img src="../../media/teloxide-core-logo.svg" width="250"/>

  <h1>teloxide-core</h1>
  <a href="https://github.com/teloxide/teloxide-core/actions">
    <img src="https://github.com/teloxide/teloxide-core/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://docs.rs/teloxide_core/">
    <img src="https://docs.rs/teloxide-core/badge.svg">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API%20coverage-Up%20to%209.2%20(inclusively)-green.svg">
  </a>
  <a href="https://crates.io/crates/teloxide_core">
    <img src="https://img.shields.io/crates/v/teloxide_core.svg">
  </a>
  <a href="https://t.me/teloxide">
    <img src="https://img.shields.io/badge/official%20chat-t.me%2Fteloxide-blueviolet">
  </a>

  The core part of [`teloxide`] providing tools for making requests to the [Telegram Bot API] with ease. This library is fully asynchronous and built using [`tokio`].
</div>

```toml
teloxide-core = "0.13.0"
```
_Compiler support: requires rustc 1.82+_.

[`teloxide`]: https://docs.rs/teloxide
[Telegram Bot API]: https://core.telegram.org/bots/api
[`tokio`]: https://tokio.rs
