
/* Steps to do:
 * - Read in file as a buffer of bytes.
 * - Convert into buffer of bits. (bitvec vs vec<bool>?)
 * - Build BitMatrix from buffer
 * - Build K2Tree from BitMatrix
 * - Serialise
 * - Save to file-system as *.k2zip
*/

/* File format:
 * - First byte is k value of tree
 * - Next byte/block is exponent to raise K to in order to get matrix width
 * - Next is the len of stems
 * - Then stems as raw bits
 * - Then len of leaves
 * - Then leaves as raw bits
 * - EOF
*/

// Load in and output files
mod filesystem;

// Build K2Tree from bytes, and compressed bytes from K2Tree
mod builder;

fn main() {
    let file_stream = filesystem::read_file::mmap_file(std::path::Path::new("hello.txt"));
    let k2tree = builder::k2_builder::k2tree_from_bytes(file_stream.data);
    let k2zip = builder::zip_builder::K2Zip::from_k2tree(k2tree);
    dbg!(&k2zip);
    let compressed_data = k2zip.clone().into_bytes();
    dbg!(&compressed_data);
    let uncompressed_k2zip = builder::zip_builder::K2Zip::from_bytes(compressed_data);
    dbg!(uncompressed_k2zip);
    println!("--- END ---");
}
