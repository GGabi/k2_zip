use memmap2::MmapOptions;
use std::fs::{OpenOptions};
use std::path::Path;
use std::slice;

//Ensuring the data doesn't outlive the mmap owning the ptr to the data
pub struct InMemFile<'a>{
    pub data:&'a[u8],
    pub mmap: memmap2::Mmap,
}

pub fn mmap_file(p: &Path) -> InMemFile {
    let file = OpenOptions::new().read(true).open(p);
    if file.is_err() {
        println!("Failed to open file: {}", p.display());
        unimplemented!();
    } else {
        println!("Succesfully openened file");
    }

    let mmap = unsafe { MmapOptions::new().map(&file.unwrap()) };
    if let Ok(mmap) = mmap {
        let len = mmap.len();
        println!("mmap'd file of {} length", len);

        let bytes = unsafe { slice::from_raw_parts(mmap.as_ptr(), len) };

        return InMemFile{
            data:bytes,
            mmap
        };
    } else {
        println!("Failed to mmap file after succesfully opening it.");
        unimplemented!();
    }
}

//Benchmarks
//For 2G file
//Safe 2.958s
//unsafe 1.015s
//291% slower
//On smaller files the difference is less.
//yes | head -n 1000000000 > bigY.txt

//For 20M file
//Unsafe 0.055s
//Safe 0.065s
//18% slower
//yes | head -n 10000000 > bigY.txt


#[test]
fn no_unsafe_bigY() {
    let mut f = File::open("bigY.txt").unwrap();

    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer);
    let mut total: u64 = 0;
    for i in 0..buffer.len() {
        total = total + buffer[i] as u64;
    }
    println!("total:{}", total);
    //assert!(total == 131000000000);
}

#[test]
fn some_unsafe_bigY() {
    let path = Path::new("bigY.txt");
    let f = mmap_file(path);
    let bytes = f.data;
    let mut total: u64 = 0;
    for i in 0..bytes.len() {
        total = total + bytes[i] as u64;
    }
    println!("total:{}", total);
    //assert!(total == 131000000000);
}
