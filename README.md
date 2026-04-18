<p align="center"><img src="frankenstein_logo.png" alt="frankenstein" height="300px"></p>

# Frakti - Frankenstein without `Send`

Frakti is a Telegram bot API client for Rust, with a focus on single-threaded async runtime support. Forked from [frankenstein](https://github.com/ayrat555/frankenstein), it is different in the following ways:

| Feature | frankenstein | frakti |
| --- | --- | --- |
| `Send` & `Sync` | Required | Not required |
| sync code | Supported | Removed |
| `async-trait` | Used | Not used |
| Multi-threaded runtime | Supported | Not supported |
| Single-threaded runtime | Need workarounds | Supported |

To summarize, frakti is your choice if you want a frankenstein alternative on a non-WASM platform and a single-threaded async runtime (like `compio`).
