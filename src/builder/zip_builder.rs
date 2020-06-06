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
    /*
      Bytes layout:
      4 - k
      4 - exp
      4 - stem_len
      ? - stems
      4 - leaf_len
      ? - leaves
    */

    //Deserialise k
    let mut remaining_k = bytes;
    let mut remaining_exp = remaining_k.split_off(4);
    let k_bytes = [remaining_k[0], remaining_k[1], remaining_k[2], remaining_k[3]];
    let k = u32::from_le_bytes(k_bytes);

      //Deserialise exp
    let mut remaining_stem_len = remaining_exp.split_off(4);
    let exp_bytes = [remaining_exp[0], remaining_exp[1], remaining_exp[2], remaining_exp[3]];
    let exp = u32::from_le_bytes(exp_bytes);

    //Deserialise stem_len
    let mut remaining_stems = remaining_stem_len.split_off(4);
    let stem_len_bytes = [remaining_stem_len[0], remaining_stem_len[1], remaining_stem_len[2], remaining_stem_len[3]];
    let stem_len = u32::from_le_bytes(stem_len_bytes);

    //Deserialise stems
    /*
      Work out how many bytes in usize,
      Work out how many stems per byte,
      Work out how many stems per usize,
      Work out how many usizes to deserialise,
      Deserialise them.
    */
    let bytes_in_usize = std::mem::size_of::<usize>();
    let stems_per_byte = 8 / (k*k) as usize;
    let stems_in_usize = bytes_in_usize * stems_per_byte;
    let usizes_to_process = ((stem_len as usize - 1) / stems_in_usize) + 1;
    let mut stems: Vec<usize> = Vec::new();
    for _ in 0..usizes_to_process {
      let mut usize_of_stems = remaining_stems;
      remaining_stems = usize_of_stems.split_off(std::mem::size_of::<usize>());
      let mut stem_bytes: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
      for byte_num in 0..stem_bytes.len() {
        stem_bytes[byte_num] = usize_of_stems[byte_num];
      }
      stems.push(usize::from_le_bytes(stem_bytes));
    }

    //Deserialise leaf_len
    let mut remaining_leaf_len = remaining_stems;
    let mut remaining_leaves = remaining_leaf_len.split_off(4);
    let leaf_len_bytes = [remaining_leaf_len[0], remaining_leaf_len[1], remaining_leaf_len[2], remaining_leaf_len[3]];
    let leaf_len = u32::from_le_bytes(leaf_len_bytes);

    //Deserialise leaves
    /*
      Work out how many bytes in usize,
      Work out how many leaves per byte,
      Work out how many leaves per usize,
      Work out how many usizes to deserialise,
      Deserialise them.
    */
    let bytes_in_usize = std::mem::size_of::<usize>();
    let leaves_per_byte = 8 / (k*k) as usize;
    let leaves_in_usize = bytes_in_usize * leaves_per_byte;
    let usizes_to_process = ((leaf_len as usize - 1) / leaves_in_usize) + 1;
    let mut leaves: Vec<usize> = Vec::new();
    for _ in 0..usizes_to_process {
      let mut usize_of_leaves = remaining_leaves;
      remaining_leaves = usize_of_leaves.split_off(std::mem::size_of::<usize>());
      let mut leaf_bytes: [u8; std::mem::size_of::<usize>()] = [0; std::mem::size_of::<usize>()];
      for byte_num in 0..leaf_bytes.len() {
        leaf_bytes[byte_num] = usize_of_leaves[byte_num];
      }
      leaves.push(usize::from_le_bytes(leaf_bytes));
    }

    //If we didn't use up all the data from file, then something went very wrong
    if remaining_leaves.len() != 0 { panic!("Bruh") }

    //Remove trailing 0s from BitVecs
    let mut stems = BitVec::from_vec(stems);
    stems.resize((stem_len*k*k) as usize, false);
    let mut leaves = BitVec::from_vec(leaves);
    leaves.resize((leaf_len*k*k) as usize, false);

    //Return
    K2Zip {
      k,
      exp,
      stem_len,
      stems,
      leaf_len,
      leaves,
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