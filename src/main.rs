#![no_std]
#![feature(start)]

mod gba_color;
mod rgb;
use gba_color::GBAColor;
use rgb::RGBDef;
use rgb::RGB;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    init_graphic();
    let vram_address: u32 = 0x06000000;
    let white: RGB = RGB::white();
    let mut offset: u32 = ((80 * 240) + 118) as u32;
    let mut vram: *mut u16 = (vram_address + (offset * 2)) as *mut u16;
    unsafe {
        *vram = white.convert_u16_color();
    }
    let dark_red: RGB = RGB::dark_red();
    offset = ((80 * 240) + 120) as u32;
    vram = (vram_address + (offset * 2)) as *mut u16;
    unsafe {
        *vram = dark_red.convert_u16_color();
    }
    let red: RGB = RGB::light_red();
    offset = ((80 * 240) + 122) as u32;
    vram = (vram_address + (offset * 2)) as *mut u16;
    unsafe {
        *vram = red.convert_u16_color();
    }
    loop {}
}

fn init_graphic() {
    let ioram_address: u32 = 0x04000000;
    let video_mode: *mut u8 = ioram_address as *mut u8;
    let bg: *mut u8 = (ioram_address + 1) as *mut u8;
    unsafe {
        *video_mode = 0x03;
        *bg = 0x04;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}
