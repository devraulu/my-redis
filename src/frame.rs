#[derive(Clone, Debug)]
enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(Debug)]
enum Error {
    Incomplete,
    Other(anyhow::Error),
}
