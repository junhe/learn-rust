use std::ptr;
use std::fs;
use std::io::{Write, SeekFrom, Seek};
use std::os::unix::prelude::AsRawFd;
use mmap::{MemoryMap, MapOption};

// from crates.io
extern crate mmap;
extern crate libc;

fn main() {
    let size: usize = 1024*1024;

    let mut f = fs::OpenOptions::new().read(true)
                                      .write(true)
                                      .create(true)
                                      .open("test.mmap")
                                      .unwrap();

    // Allocate space in the file first
    f.seek(SeekFrom::Start(size as u64)).unwrap();
    f.write_all(&[0]).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    let mmap_opts = &[
        // Then make the mapping *public* so it is written back to the file
        MapOption::MapNonStandardFlags(libc::MAP_SHARED),
        MapOption::MapReadable,
        MapOption::MapWritable,
        MapOption::MapFd(f.as_raw_fd()),
    ];

    let mmap = MemoryMap::new(size, mmap_opts).unwrap();

    let data = mmap.data();

    if data.is_null() {
        panic!("Could not access data from memory mapped file")
    }

    let src = "Hello!";
    let src_data = src.as_bytes();

    let mdata = data as *mut libc::c_void;


    unsafe {
        libc::madvise(mdata, 4096, libc::MADV_RANDOM);
        ptr::copy(src_data.as_ptr(), data, src_data.len());
    }
}









