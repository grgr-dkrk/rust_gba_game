pub struct Io {
  io_address: u32,
}

impl Io {
  pub fn new() -> Self {
    Io {
      io_address: 0x04000000,
    }
  }
  pub fn screen_mode3_init(&self) {
    let video_mode: *mut u8 = self.io_address as *mut u8;
    let bg: *mut u8 = (self.io_address + 1) as *mut u8;
    unsafe {
      *video_mode = 0x03;
      *bg = 0x04;
    }
  }
}
