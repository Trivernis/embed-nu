use nu_protocol::Span;

pub trait SpanEmpty {
    fn empty() -> Self;
}

impl SpanEmpty for Span {
    fn empty() -> Self {
        Span::new(0, 0)
    }
}
