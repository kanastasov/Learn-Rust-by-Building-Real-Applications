use std::collections::HashMap;

pub struct QueryString<'buf>{
    data: HashMap<& 'buf str, & 'buf str>
}

pub enum value {
    Single(& 'buf str),
    Multiple(Vec<& 'buf str>),
}