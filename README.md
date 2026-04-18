<p align="center"><img src="frankenstein_logo.png" alt="frankenstein" height="300px"></p>

# Frakti - Frankenstein without `Send`

Frakti is a fork of [frankenstein](https://github.com/ayrat555/frankenstein) crate, with a focus on single-threaded async runtime support. To be specific:

| Feature | frankenstein | frakti |
| --- | --- | --- |
| `Send` & `Sync` | Required | Not required |
| sync code | Supported | Removed |
| `async-trait` | Used | Not used |
| Multi-threaded runtime | Supported | Not supported |
| Single-threaded runtime | Need workarounds | Supported |
