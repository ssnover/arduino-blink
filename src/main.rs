#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use avr_device::atmega328p::Peripherals;
use core::arch::asm;
use panic_halt as _;

const F_CPU: u32 = 16_000_000;

#[no_mangle]
pub extern fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let portb = peripherals.PORTB;
    portb.ddrb.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5))});

    loop {
        portb.portb.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5))});

        portb.portb.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5) )});
    }
}

fn delay_ms(ms: u16) {
    let ticks = (F_CPU / 4000) as u16;
    for _ in 0..(F_CPU / 4000) {
        unsafe { asm!("nop", "nop", "nop", "nop"); }
    }
}