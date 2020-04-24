#![no_std]
#![feature(start)]

mod color;
mod picture;
mod screen;
mod vram;

use picture::SAMPLE_PICT;
use screen::Screen;
use vram::Vram;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let screen = Screen::new();
    screen.mode3_init();

    let vram: Vram = Vram::new();
    vram.paint_pict(SAMPLE_PICT);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}
