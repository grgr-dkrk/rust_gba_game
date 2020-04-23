#![no_std]
#![feature(start)]

mod gba_color;
mod graphics;
mod picture;
mod rgb;
mod screen;

use graphics::Graphics;
use picture::SAMPLE_PICT;
use screen::Screen;
// use rgb::RGBDef;
// use rgb::RGB;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let screen = Screen::new();
    screen.mode3_init();

    let graphics: Graphics = Graphics::new();
    graphics.paint_pict(SAMPLE_PICT);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}
