#![no_std]
#![feature(start)]

mod gba_color;
mod graphics;
mod picture;
mod rgb;

use graphics::Graphics;
use picture::SAMPLE_PICT;
// use rgb::RGBDef;
// use rgb::RGB;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    init_graphic();

    let graphics: Graphics = Graphics::new();
    graphics.paint_pict(SAMPLE_PICT);

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
