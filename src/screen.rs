pub struct Screen {
  iwram_address: u32,
}

impl Screen {
  pub fn new() -> Self {
    Screen {
      iwram_address: 0x04000000,
    }
  }
  pub fn mode3_init(&self) {
    let video_mode: *mut u8 = self.iwram_address as *mut u8;
    let bg: *mut u8 = (self.iwram_address + 1) as *mut u8;
    unsafe {
      *video_mode = 0x03;
      *bg = 0x04;
    }
  }
}
