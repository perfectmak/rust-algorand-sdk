// helper trait for moving from slices to fixed array
pub trait ToArray<T> {
  fn to_array(&self) -> T;
}

impl ToArray<[u8; 32]> for &[u8] {
  fn to_array(&self) -> [u8; 32] {
    if self.len() < 32 {
      panic!("wrong slice length. slice must be 32 in length, got {}", self.len());
    }
    let mut result_bytes: [u8; 32] = [0; 32];
    result_bytes.copy_from_slice(self);
    result_bytes
  }
}

impl ToArray<[u8; 32]> for Vec<u8> {
  fn to_array(&self) -> [u8; 32] {
    if self.len() < 32 {
      panic!("wrong vector length. vector must be 32 in length, got {}", self.len());
    }
    let mut result_bytes: [u8; 32] = [0; 32];
    result_bytes.copy_from_slice(self.as_ref());
    result_bytes
  }
}

impl ToArray<[u8; 64]> for &[u8] {
  fn to_array(&self) -> [u8; 64] {
    if self.len() < 64 {
      panic!("wrong slice length. slice must be 64 in length, got {}", self.len());
    }
    let mut result_bytes: [u8; 64] = [0; 64];
    result_bytes.copy_from_slice(self);
    result_bytes
  }
}