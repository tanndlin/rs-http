pub trait EncodeTo {
    fn encode_to(self, buf: &mut Vec<u8>);

    fn to_bytes(self) -> Vec<u8>
    where
        Self: std::marker::Sized,
    {
        let mut buf = Vec::new();
        self.encode_to(&mut buf);
        buf
    }
}
