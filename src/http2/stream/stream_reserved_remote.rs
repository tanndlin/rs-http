use crate::http2::{
    connection_state::ConnectionState, error::HTTP2Error, frames::frame::Frame,
    stream::http_stream::HTTP2Stream,
};

#[derive(Clone, Debug)]
pub struct HTTP2StreamReservedRemote {
    _id: u32,
}

impl HTTP2StreamReservedRemote {
    pub fn handle_frame(
        self,
        _frame: Frame,
        _state: &mut ConnectionState<'_>,
    ) -> Result<(HTTP2Stream, Vec<Frame>), (HTTP2Stream, HTTP2Error)> {
        todo!("Implement handle_frame for reserved (remote) stream")
    }
}

impl From<HTTP2StreamReservedRemote> for HTTP2Stream {
    fn from(stream: HTTP2StreamReservedRemote) -> Self {
        HTTP2Stream::ReservedRemote(stream)
    }
}
