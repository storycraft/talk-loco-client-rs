# TalkLocoClient
Official client/server compatible Loco client implementation

## Contributing

### Command
Check `src/request`, `src/response` directory for already implemented command datas.
For data structs used in many places check `src/structs`.

Example command data implementation below
```rust
use serde::{Serialize, Deserialize};

// Add `Req` suffix to request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleDataReq {
    /// Alias for obfuscated key
    #[serde(rename = "req")]
    pub request: String
}


// Add `Res` suffix to response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleDataRes {
    #[serde(rename = "res")]
    pub response: String
}
```

### Word convention
* chat room, channel, ... => channel
* member, user, ... => user
* message, chat, ... => chat

## License
TalkLocoClient is following MIT license.

See `LICENSE` for full text.