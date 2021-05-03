pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(data_file: File) -> io::Result<Self> {
        // TODO
    }

    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
        // TODO
    }

    pub fn allocate_page(&mut self) -> PageId {
        // TODO
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        // TODO
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        // TODO
    }
}