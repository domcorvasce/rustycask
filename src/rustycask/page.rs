use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub struct Page {
    block_size: u64,
    block_start_pos: usize,
    contents: Vec<u8>,
}

impl Page {
    fn new(block_size: u64) -> Self {
        Self {
            block_size,
            block_start_pos: block_size as usize - 1,
            contents: vec![0; block_size.try_into().unwrap()],
        }
    }

    fn write(&mut self, stream: &[u8]) -> Result<(), &str> {
        let stream_size = stream.len();
        let stream_len_size = size_of::<usize>();
        if self.block_start_pos - size_of::<usize>() < stream_size + stream_len_size {
            return Err("page is full");
        }

        let start_pos = self.block_start_pos - stream_len_size - stream_size;

        self.write_at(stream, start_pos);
        self.block_start_pos = start_pos;
        self.write_at(&start_pos.to_le_bytes(), 0);
        Ok(())
    }

    fn write_at(&mut self, stream: &[u8], pos: usize) {
        let stream_size = stream.len();
        let stream_len_size = size_of::<usize>();

        self.contents[pos..pos + stream_len_size].copy_from_slice(&stream_size.to_le_bytes());
        self.contents[pos + stream_len_size..pos + stream_len_size + stream_size]
            .copy_from_slice(stream);
    }

    fn load(&mut self, file: &mut File, block_id: u64) {
        let block_relative_pos = SeekFrom::Start(block_id * self.block_size);

        file.seek(block_relative_pos).unwrap();
        file.read_exact(&mut self.contents).unwrap();

        self.block_start_pos =
            usize::from_le_bytes(self.contents[0..size_of::<usize>()].try_into().unwrap());
    }

    fn flush(&self, file: &mut File, block_id: u64) {
        let block_relative_pos = SeekFrom::Start(block_id * self.block_size);

        file.seek(block_relative_pos).unwrap();
        file.write_all(&self.contents).unwrap();
        file.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn test_init_page() {
        let page = Page::new(4096);
        assert_eq!(page.contents.len(), 4096);
    }

    #[test]
    fn test_append_data_to_page() {
        let mut page = Page::new(4096);
        assert_eq!(page.block_start_pos, 4095);

        page.write("hello world".as_bytes()).unwrap();
        assert_eq!(page.block_start_pos, 4076);
    }

    #[test]
    fn test_return_error_on_appending_data() {
        let mut page = Page::new(50);
        let res = page.write("hello world".as_bytes());
        assert!(res.is_ok());

        let res2 = page.write("hello darkness my old friend".as_bytes());
        assert!(res2.is_err());
    }

    #[test]
    fn test_load_page_from_file() {
        let mut file = tempfile::tempfile().unwrap();
        let mut page = Page::new(4096);
        let mut page2 = Page::new(4096);

        page.write("hello".as_bytes()).unwrap();
        page.flush(&mut file, 0);

        page2.load(&mut file, 0);
        assert_eq!(page.block_start_pos, 4082);
    }

    #[test]
    fn test_flush_page_to_file() {
        let mut file = tempfile::tempfile().unwrap();
        let page = Page::new(4096);
        page.flush(&mut file, 0);

        let file_metadata = file.metadata().unwrap();
        assert_eq!(file_metadata.len(), 4096);
    }
}
