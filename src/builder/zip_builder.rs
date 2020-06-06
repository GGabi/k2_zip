use k2_tree::K2Tree;
use bitvec::vec::BitVec;

#[derive(Debug, Clone)]
pub struct K2Zip {
  pub k: u32,
  pub exp: u32, //to get matrix_width from k
  pub stem_len: u32, //number of stems
  pub stems: BitVec,
  pub leaf_len: u32, //number of leaves
  pub leaves: BitVec,
}

impl K2Zip {
  pub fn from_k2tree(k2tree: K2Tree) -> Self {
    let k = k2tree.k; //It's used a lot, increases clarity and brevity
    K2Zip {
      k: k as u32,
      exp: (k2tree.matrix_width as f64).log(k as f64) as u32,
      stem_len: (k2tree.stems.len() / (k*k)) as u32,
      stems: k2tree.stems,
      leaf_len: (k2tree.leaves.len() / (k*k)) as u32,
      leaves: k2tree.leaves,
    }
  }
  pub fn from_bytes(bytes: Vec<u8>) -> Self {
    //first 4 - k
    //4 - exp
    //4 - stem_len
    //? - stems
    //4 - leaf_len
    //? - leaves
    let mut remaining_k = bytes;

    let mut remaining_exp = remaining_k.split_off(4);
    let k_bytes = [remaining_k[0], remaining_k[1], remaining_k[2], remaining_k[3]];
    let k = u32::from_le_bytes(k_bytes);

    let mut remaining_stem_len = remaining_exp.split_off(4);
    let exp_bytes = [remaining_exp[0], remaining_exp[1], remaining_exp[2], remaining_exp[3]];
    let exp = u32::from_le_bytes(exp_bytes);

    let mut remaining_stems = remaining_stem_len.split_off(4);
    let stem_len_bytes = [remaining_stem_len[0], remaining_stem_len[1], remaining_stem_len[2], remaining_stem_len[3]];
    let stem_len = u32::from_le_bytes(stem_len_bytes);

    let mut stems: Vec<usize> = Vec::new();
    for stem_num in 0..stem_len {
      let stem = remaining_stems.split_off(std::mem::size_of::<usize>());
      let mut stem_bytes: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
      for byte_num in 0..std::mem::size_of::<usize>() {
        stem_bytes[byte_num] = stem[byte_num];
      }
      stems.push(usize::from_le_bytes(stem_bytes));
    }

    let mut remaining_leaf_len = remaining_stems;
    let mut remaining_leaves = remaining_leaf_len.split_off(4);
    let leaf_len_bytes = [remaining_leaf_len[0], remaining_leaf_len[1], remaining_leaf_len[2], remaining_leaf_len[3]];
    let leaf_len = u32::from_le_bytes(leaf_len_bytes);

    let mut leaves: Vec<usize> = Vec::new();
    for leaf_num in 0..stem_len {
      let leaf = remaining_leaves.split_off(std::mem::size_of::<usize>());
      let mut leaf_bytes: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
      for byte_num in 0..std::mem::size_of::<usize>() {
        leaf_bytes[byte_num] = leaf[byte_num];
      }
      leaves.push(usize::from_le_bytes(leaf_bytes));
    }

    if remaining_leaves.len() != 0 { panic!("Bruh") }

    K2Zip {
      k,
      exp,
      stem_len,
      stems: BitVec::from_vec(stems),
      leaf_len,
      leaves: BitVec::from_vec(leaves),
    }
  }
  pub fn into_bytes(self) -> Vec<u8> {
    //Should it be Little Endian?
    let mut buf: Vec<u8> = Vec::new();
    buf.extend(self.k.to_le_bytes().iter());
    buf.extend(self.exp.to_le_bytes().iter());
    buf.extend(self.stem_len.to_le_bytes().iter());
    buf.extend(
      self.stems
      .into_vec()
      .iter()
      .map(|n| n.to_le_bytes().to_vec())
      .flatten()
      .collect::<Vec<u8>>()
    );
    buf.extend(self.leaf_len.to_le_bytes().iter());
    buf.extend(
      self.leaves
      .into_vec()
      .iter()
      .map(|n| n.to_le_bytes().to_vec())
      .flatten()
      .collect::<Vec<u8>>()
    );
    buf
  }
}

//Lsb0 Msb0
//6:
//Lsb0 => [011.......] ([usize])
//Msb0 => [1, 1, 0]