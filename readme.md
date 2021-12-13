# TalkLocoClient
Official client/server compatible Loco client implementation

## Contributing

### Command
See `src/request`, `src/response` directory for already implemented command datas.
For data structs used in many places see `src/structs`.

Example command data implementation.
```rust
use serde::{Serialize, Deserialize};

// Add `Req` suffix to request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleDataReq {
    pub request: String
}


// Add `Res` suffix to response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleDataRes {
    pub response: String
}
```

### Word convention
chat room, channel, ... => channel
member, user, ... => user
message, chat, ... => chat

## License
TalkLocoClient is following MIT license.

See `LICENSE` for full text.