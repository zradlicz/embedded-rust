#![no_main]
#![no_std]

use blink as _; // global logger + panicking-behavior + memory layout

use stm32g4_drivers::reg;
use stm32g4_drivers::systick;
use stm32g4_drivers::gpio;
use stm32g4_drivers::timer;

#[cortex_m_rt::entry]
fn main() -> ! {

    systick::systick_init();
    gpio::a_init();

    defmt::println!("Starting blinking LED with Custom Bare Metal Register Access");

    let mut blinker = timer::Timer::new(1_000_000, true, || gpio::a_toggle_pin(5));

    blinker.start();

    loop
    {
        blinker.check();
    }

    blink::exit()
}
