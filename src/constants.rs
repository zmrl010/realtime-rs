pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// pub const DEFAULT_HEADERS:  = 0

pub const DEFAULT_TIMEOUT: u32 = 10000;

pub const WS_CLOSE_NORMAL: u32 = 1000;

pub enum SocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

pub enum ChannelState {
    Closed,
    Errored,
    Joined,
    Joining,
    Leaving,
}

pub enum ChannelEvent {
    Close,
    Error,
    Join,
    Reply,
    Leave,
    AccessToken,
}

pub enum Transports {
    Websocket,
}

pub enum ConnectionState {
    Connecting,
    Open,
    Closing,
    Closed,
}
