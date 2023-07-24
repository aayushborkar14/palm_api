# PaLM API

[![crates.io](https://img.shields.io/crates/v/palm_api.svg)](https://crates.io/crates/palm_api)
[![Documentation](https://docs.rs/palm_api/badge.svg)](https://docs.rs/palm_api)
![MIT/Apache-2 licensed](https://img.shields.io/crates/l/reqwest.svg)

Get started using the PaLM API in Rust.

## Usage

Get an [API key from MakerSuite](https://makersuite.google.com/app/apikey), then configure it here.
```rust,no_run
use palm_api::palm::create_client;

client = create_client(PALM_API_KEY.to_string());
```

Use `PalmClient`'s `generate_text()` method to have the model complete some initial text.
```rust,no_run
use palm_api::palm::new_text_body;

let mut text_body = new_text_body();
text_body.set_text_prompt("The opposite of hot is".to_string());
let response = client
    .generate_text("text-bison-001".to_string(), text_body)
    .expect("An error has occured.");
println!("{}", response.candidates.unwrap()[0].output);
```

Use `PalmClient`'s `chat()` method to have a discussion with a model.
```rust,no_run
use palm_api::palm::new_chat_body;

let mut chat_body = new_chat_body();
chat_body.append_message("Hello.".to_string());
let response = client
    .chat("chat-bison-001".to_string(), chat_body)
    .expect("An error has occured.");
println!("{}", response.candidates.unwrap()[0].content);
```

