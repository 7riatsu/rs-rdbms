use anyhow::Result;

use rs_rdbms::btree::{BTree, SearchMode};
use rs_rdbms::buffer::{BufferPool, BufferPoolManager};
use rs_rdbms::disk::{DiskManager, PageId};

fn main() -> Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut bufmgr, SearchMode::Key(b"Gifu".to_vec()))?;
    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        println!("{:02x?} = {:02x?}", key, value);
    }
    Ok(())
}
