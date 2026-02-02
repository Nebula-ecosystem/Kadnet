#[derive(Debug)]
pub enum NetworkError {
    Connection,
    Timeout,
    Write,
    Read,
    CouldNotParseRPC,
    Send,
}
