use crate::http2::{
    error::{HTTP2Error, HTTP2ErrorCode},
    frames::{
        data_frame::DataFrame, go_away_frame::GoAwayFrame, headers_frame::HeadersFrame,
        ping_frame::PingFrame, priority_frame::PriorityFrame, push_promise_frame::PushPromiseFrame,
        rst_frame::RstFrame, settings_frame::SettingsFrame, window_update_frame::WindowUpdateFrame,
    },
};

#[repr(u8)]
#[derive(Debug, Default)]
pub enum FrameType {
    #[default]
    Data = 0,
    Headers = 1,
    Priority = 2,
    RstStream = 3,
    Settings = 4,
    PushPromise = 5,
    Ping = 6,
    GoAway = 7,
    WindowUpdate = 8,
}

impl From<u8> for FrameType {
    fn from(value: u8) -> Self {
        match value {
            1 => FrameType::Headers,
            2 => FrameType::Priority,
            3 => FrameType::RstStream,
            4 => FrameType::Settings,
            5 => FrameType::PushPromise,
            6 => FrameType::Ping,
            7 => FrameType::GoAway,
            8 => FrameType::WindowUpdate,
            _ => FrameType::Data,
        }
    }
}

#[derive(Debug)]
pub enum Frame {
    Data(DataFrame),
    Headers(HeadersFrame),
    Priority(PriorityFrame),
    RstStream(RstFrame),
    Settings(SettingsFrame),
    PushPromise(PushPromiseFrame),
    Ping(PingFrame),
    GoAway(GoAwayFrame),
    WindowUpdate(WindowUpdateFrame),
}

impl TryFrom<&[u8]> for Frame {
    type Error = HTTP2Error;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        assert!(
            buf.len() >= 9,
            "Tried to parse frame but buffer was less than 9 bytes for frame header"
        );

        let frame_type = FrameType::from(buf[3]);
        Ok(match frame_type {
            FrameType::Data => Frame::Data(
                DataFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::Headers => Frame::Headers(
                HeadersFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::Priority => Frame::Priority(
                PriorityFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::RstStream => Frame::RstStream(
                RstFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::Settings => Frame::Settings(
                SettingsFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::PushPromise => Frame::PushPromise(
                PushPromiseFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::Ping => Frame::Ping(
                PingFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::GoAway => Frame::GoAway(
                GoAwayFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
            FrameType::WindowUpdate => Frame::WindowUpdate(
                WindowUpdateFrame::try_from(buf)
                    .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?,
            ),
        })
    }
}

impl Frame {
    pub fn get_stream_id(&self) -> u32 {
        match self {
            Frame::Data(f) => f.header.stream_id,
            Frame::Headers(f) => f.header.stream_id,
            Frame::Priority(f) => f.header.stream_id,
            Frame::RstStream(f) => f.header.stream_id,
            Frame::Settings(f) => f.header.stream_id,
            Frame::PushPromise(f) => f.header.stream_id,
            Frame::Ping(f) => f.header.stream_id,
            Frame::GoAway(f) => f.header.stream_id,
            Frame::WindowUpdate(f) => f.header.stream_id,
        }
    }
}

#[derive(Debug, Default)]
pub struct FrameHeader<T>
where
    T: From<u8>,
{
    pub length: u32,           //24 bits
    pub frame_type: FrameType, // 8 bits
    pub flags: T,
    pub stream_id: u32, // 31 bits (R infront)
}

impl<T> From<FrameHeader<T>> for Vec<u8>
where
    T: From<u8>,
    T: Into<u8>,
{
    #[allow(clippy::cast_possible_truncation)]
    fn from(val: FrameHeader<T>) -> Self {
        let mut buf = vec![
            (val.length >> 16) as u8,
            (val.length >> 8) as u8,
            val.length as u8,
            val.frame_type as u8,
            val.flags.into(),
        ];
        buf.extend_from_slice(&val.stream_id.to_be_bytes());
        buf
    }
}

impl<T> TryFrom<&[u8]> for FrameHeader<T>
where
    T: From<u8>,
{
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 9 {
            return Err("Frame header must be at least 9 bytes".to_string());
        }

        let length = (u32::from(buf[0]) << 16) | (u32::from(buf[1]) << 8) | u32::from(buf[2]);
        let frame_type = FrameType::from(buf[3]);
        let flag_bits = buf[4];
        let flags = T::from(flag_bits);
        let stream_identifier = u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]]) & !(0b1 << 31);

        Ok(Self {
            length,
            frame_type,
            flags,
            stream_id: stream_identifier,
        })
    }
}
