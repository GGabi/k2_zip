
fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
  let mut bits = Vec::with_capacity(bytes.len() * 8);
  for byte in bytes {
    bits.extend(byte_to_bits(*byte).iter());
  }
  bits
}

fn byte_to_bits(byte: u8) -> [bool; 8] {
  [
    byte & 0b10000000 != 0,
    byte & 0b01000000 != 0,
    byte & 0b00100000 != 0,
    byte & 0b00010000 != 0,
    byte & 0b00001000 != 0,
    byte & 0b00000100 != 0,
    byte & 0b00000010 != 0,
    byte & 0b00000001 != 0,
  ]
}

pub fn k2tree_from_bytes(bytes: &[u8]) -> k2_tree::K2Tree {
  let bits = bytes_to_bits(bytes);
  let raw_width = (bits.len() as f64).sqrt();
  let power_of_2 = raw_width.log(2.0) as i32;
  let valid_width = 2_f64.powi(power_of_2+1) as usize;
  let valid_height = valid_width;
  let bit_matrix = k2_tree::matrix::BitMatrix::from_bits(
    valid_width,
    valid_height,
    bits
  );
  k2_tree::K2Tree::from_matrix(bit_matrix).unwrap()
}