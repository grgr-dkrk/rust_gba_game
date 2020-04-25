pub struct Io {
  video_mode: *mut u8,
  bg_mode: *mut u8,
}

impl Io {
  pub fn new() -> Self {
    Io {
      video_mode: 0x04000000 as *mut u8,
      bg_mode: (0x04000000 + 1) as *mut u8,
    }
  }
  pub fn screen_mode3_init(&self) {
    unsafe {
      *self.video_mode = 0x03;
      *self.bg_mode = 0x04;
    }
  }
  pub fn screen_mode4_init(&self) {
    unsafe {
      *self.video_mode = 0x04;
      *self.bg_mode = 0x04;
    }
  }
}
