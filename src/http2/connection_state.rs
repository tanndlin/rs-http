use hpack::{Decoder, Encoder};

pub struct ConnectionState<'a> {
    pub decoder: Decoder<'a>,
    pub encoder: Encoder<'a>,
    pub settings_acked: bool,
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
        }
    }
}
