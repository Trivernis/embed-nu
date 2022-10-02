use std::collections::HashMap;

use nu_protocol::{Span, Value};

pub trait SpanEmpty {
    fn empty() -> Self;
}

impl SpanEmpty for Span {
    fn empty() -> Self {
        Span::new(0, 0)
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String {
            val: self,
            span: Span::empty(),
        }
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Int {
            val: self,
            span: Span::empty(),
        }
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool {
            val: self,
            span: Span::empty(),
        }
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Float {
            val: self,
            span: Span::empty(),
        }
    }
}

impl IntoValue for char {
    fn into_value(self) -> Value {
        Value::String {
            val: self.to_string(),
            span: Span::empty(),
        }
    }
}

impl<'a> IntoValue for &'a str {
    fn into_value(self) -> Value {
        self.to_string().into_value()
    }
}

impl IntoValue for HashMap<String, Value> {
    fn into_value(self) -> Value {
        let (cols, vals) = self.into_iter().unzip();

        Value::Record {
            cols,
            vals,
            span: Span::empty(),
        }
    }
}

impl IntoValue for Vec<Value> {
    fn into_value(self) -> Value {
        Value::List {
            vals: self,
            span: Span::empty(),
        }
    }
}
