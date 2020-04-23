// original: https://booth.pm/ja/items/492956

use crate::gba_color::GBAColor;
use crate::rgb::RGB;

pub struct Graphics {
  vram_address: u32,
  screen_x: u16,
  screen_y: u16,
}

impl Graphics {
  pub fn new() -> Self {
    Graphics {
      vram_address: 0x06000000,
      screen_x: 240,
      screen_y: 160,
    }
  }

  pub fn draw_dot(&self, x: u16, y: u16, color: &RGB) {
    let offset: u32 = ((y * self.screen_x) + x) as u32;
    let vram: *mut u16 = (self.vram_address + (offset * 2)) as *mut u16;
    unsafe {
      *vram = color.convert_u16_color();
    }
  }

  pub fn draw_box(&self, x: u16, y: u16, width: u16, height: u16, color: &RGB) {
    for offset_y in 0..height {
      for offset_x in 0..width {
        if (x + offset_x > self.screen_x) || (y + offset_y > self.screen_y) {
          continue;
        }
        self.draw_dot(x + offset_x, y + offset_y, color)
      }
    }
  }

  pub fn draw_circle(&self, center_x: u16, center_y: u16, r: u16, color: &RGB) {
    let mut x: u16 = r;
    let mut y: u16 = 0;
    let mut f: i32 = 3 - ((r as i32) * 2);

    loop {
      if x < y {
        break;
      }

      self.draw_dot(center_x + x, center_y + y, color);
      self.draw_dot(center_x - x, center_y + y, color);
      self.draw_dot(center_x + x, center_y - y, color);
      self.draw_dot(center_x - x, center_y - y, color);
      self.draw_dot(center_x + x, center_y + y, color);
      self.draw_dot(center_x - x, center_y + y, color);
      self.draw_dot(center_x + x, center_y - y, color);
      self.draw_dot(center_x - x, center_y - y, color);

      if f >= 0 {
        x -= 1;
        f -= (x * 4) as i32;
      }
      y += 1;
      f += (4 * y + 2) as i32;
    }
  }

  pub fn paint_all(&self, color: &RGB) {
    for y in 0..self.screen_y {
      for x in 0..self.screen_x {
        self.draw_dot(x, y, color);
      }
    }
  }

  pub fn paint_pict(&self, pict: [u16; 38400]) {
    for y in 0..self.screen_y {
      for x in 0..self.screen_x {
        unsafe {
          (self.vram_address as *mut u16)
            .offset((y * self.screen_x + x) as isize)
            .write_volatile(pict[(y * self.screen_x + x) as usize])
        }
      }
    }
  }
}
