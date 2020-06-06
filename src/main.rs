
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
    println!("Hello, world!");
}
