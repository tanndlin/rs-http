pub enum StreamState {
    Idle,
    Open,
    ReservedLocal,
    ReservedRemote,
    HalfClosedRemote,
    HalfClosedLocal,
    Closed,
}

pub struct HTTP2Stream {
    pub identifier: u32,
    pub state: StreamState,
}

impl HTTP2Stream {
    pub fn new(identifier: u32) -> Self {
        HTTP2Stream {
            identifier,
            state: StreamState::Idle,
        }
    }
}
