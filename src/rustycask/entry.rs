pub struct Entry<'a> {
    key: &'a [u8],
    value: &'a [u8],
    key_size: u64,
    value_size: u64,
}

impl<'a> Entry<'a> {
    fn new(key: &'a str, value: &'a str) -> Self {
        Self {
            key: key.as_bytes(),
            value: value.as_bytes(),
            key_size: key.len().try_into().unwrap(),
            value_size: value.len().try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn test_construct_entry() {
        let entry = Entry::new("name", "John");
        assert_eq!(entry.key_size, 4);
        assert_eq!(entry.value_size, 4);
    }
}
