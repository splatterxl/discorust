#[derive(Debug)]
pub struct OpcodeListener {
  opcode: u8
}

impl OpcodeListener {
  pub fn call(&self, _data: crate::util::Object) {
    panic!("Called unconfigured opcode listener");
  }
}
