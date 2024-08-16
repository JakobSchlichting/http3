#[derive(Debug)]
enum QuicError {
    ProtocolViolation(String),
}
