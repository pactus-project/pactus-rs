pub enum Lazy<T> {
    Raw(Vec<u8>),
    Decoded(T),
}
