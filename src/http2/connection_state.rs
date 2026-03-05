use hpack::{Decoder, Encoder};

pub struct ConnectionSettings {
    pub window_size: u32,
}

pub struct ConnectionState<'a> {
    pub decoder: Decoder<'a>,
    pub encoder: Encoder<'a>,
    pub settings_acked: bool,
    pub settings_sent: bool,
    pub settings: ConnectionSettings,
}

impl ConnectionState<'_> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ConnectionState<'_> {
    fn default() -> Self {
        ConnectionState {
            decoder: Decoder::new(),
            encoder: Encoder::new(),
            settings_acked: true,
            settings_sent: false,
            settings: ConnectionSettings::default(),
        }
    }
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        ConnectionSettings { window_size: 65535 }
    }
}
