pub struct RealtimeChannelConfigBroadcast {
    /// enables client to receive message it broadcast
    pub self_emit: Option<bool>,
    /// instructs server to acknowledge that broadcast message was received
    pub ack: Option<bool>,
}

pub struct RealtimeChannelConfigPresence {
    pub key: Option<String>,
}

pub struct RealtimeChannelConfig {
    pub broadcast: Option<RealtimeChannelConfigBroadcast>,
    pub presence: Option<RealtimeChannelConfigPresence>,
}

pub struct RealtimeChannelOptions {
    pub config: RealtimeChannelConfig,
}

pub struct RealtimeChannel {}
