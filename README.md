# Chat

A simple implementation of a chat with gpt-3.5-turbo.

Technology stack:
- [Rust](https://www.rust-lang.org/) used for backend implementation,
- [Slint UI framework](https://slint.dev/) used for the user interface implementation,
- [tokio](https://docs.rs/tokio/latest/tokio/index.html) used as a runtime for event-driven network communication,
- [openai](https://crates.io/crates/openai) used for accessing the OpenAI API.

## Build

It can be built using `cargo` tool.

```
$ cargo build
```

## Run

This application uses the `OPENAI_API_KEY` environment variable to connect with the OpenAI models.

```
$ export OPENAI_API_KEY="<your-openai-api-key>"
$ cargo run
```
