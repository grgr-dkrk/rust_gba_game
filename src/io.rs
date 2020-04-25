pub struct Io {
  video_mode: *mut u8,
  bg_mode: *mut u8,
  v_count: usize,
}

impl Io {
  pub fn new() -> Self {
    Io {
      video_mode: 0x04000000 as *mut u8,
      bg_mode: (0x04000000 + 1) as *mut u8,
      v_count: 0x0400_0006,
    }
  }
  pub fn wait_for_vsnc(&self) {
    unsafe {
      while core::ptr::read_volatile(self.v_count as *const u32) >= 160 {}
      while core::ptr::read_volatile(self.v_count as *const u32) > 160 {}
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
