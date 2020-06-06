
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
  pub fn into_bytes(self) -> Vec<u8> {
    //Should it be Big Endian?
    let mut buf: Vec<u8> = Vec::new();
    buf.extend(self.k.to_ne_bytes().iter());
    buf.extend(self.exp.to_ne_bytes().iter());
    buf.extend(self.stem_len.to_ne_bytes().iter());
    buf.extend(
      self.stems
      .into_vec()
      .iter()
      .map(|n| n.to_ne_bytes().to_vec())
      .flatten()
      .collect::<Vec<u8>>()
    );
    buf.extend(self.leaf_len.to_ne_bytes().iter());
    buf.extend(
      self.leaves
      .into_vec()
      .iter()
      .map(|n| n.to_ne_bytes().to_vec())
      .flatten()
      .collect::<Vec<u8>>()
    );
    buf
  }
}