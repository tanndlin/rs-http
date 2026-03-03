use crate::http2::{
    error::HTTP2Error,
    frames::frame::Frame,
    stream::{stream_closed::HTTP2StreamClosed, stream_idle::HTTP2StreamIdle},
};

pub enum HTTP2Stream {
    Idle(HTTP2StreamIdle),
    Open,
    ReservedLocal,
    ReservedRemote,
    HalfClosedRemote,
    HalfClosedLocal,
    Closed(HTTP2StreamClosed),
}

impl HTTP2Stream {
    pub fn handle_frame(&self, frame: Frame) -> Result<Vec<u8>, HTTP2Error> {
        match self {
            HTTP2Stream::Idle(http2_stream_idle) => http2_stream_idle.handle_frame(frame),
            HTTP2Stream::Open => todo!(),
            HTTP2Stream::ReservedLocal => todo!(),
            HTTP2Stream::ReservedRemote => todo!(),
            HTTP2Stream::HalfClosedRemote => todo!(),
            HTTP2Stream::HalfClosedLocal => todo!(),
            HTTP2Stream::Closed => todo!(),
        }
    }
}

impl HTTP2Stream {
    pub fn new(identifier: u32) -> Self {
        HTTP2Stream::Idle(HTTP2StreamIdle { identifier })
    }
}
