use anyhow::Result;

use rs_rdbms::buffer::{BufferPool, BufferPoolManager};
use rs_rdbms::disk::{DiskManager, PageId};
use rs_rdbms::table::{Table, UniqueIndex};

fn main() -> Result<()> {
    // let disk = DiskManager::open("simple.rly")?;
    let disk = DiskManager::open("table.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    // let mut table = SimpleTable {
    //     meta_page_id: PageId(0),
    //     num_key_elems: 1,
    // };

    let mut table = Table {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
        unique_indices: vec![UniqueIndex {
            meta_page_id: PageId::INVALID_PAGE_ID,
            skey: vec![2],
        }],
    };

    table.create(&mut bufmgr)?;
    dbg!(&table);
    table.insert(&mut bufmgr, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut bufmgr, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut bufmgr, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut bufmgr, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut bufmgr, &[b"v", b"Eve", b"Brown"])?;
    bufmgr.flush()?;
    Ok(())
}
