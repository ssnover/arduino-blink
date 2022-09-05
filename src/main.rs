#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use avr_device::atmega328p::Peripherals;
use core::arch::asm;

extern crate avr_std_stub;

const F_CPU: u32 = 16_000_000;

#[avr_device::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let portb = peripherals.PORTB;
    portb
        .ddrb
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });

    loop {
        portb
            .portb
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });
        delay_ms(1000);
        portb
            .portb
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });
        delay_ms(1000);
    }
}

fn delay_ms(ms: u32) {
    for _ in 0..(F_CPU / 4000 * ms) {
        unsafe {
            asm!("nop", "nop", "nop", "nop");
        }
    }
}
