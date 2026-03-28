#![no_main]
#![no_std]

use bootloader as _; // global logger + panicking-behavior + memory layout

use stm32g4_drivers::reg;
use stm32g4_drivers::uart;
use stm32g4_drivers::gpio;
use core::arch::asm;

const SP_ADDRESS : *mut u32 = 0x08008000 as *mut u32;
const PC_ADDRESS : *mut u32 = 0x08008004 as *mut u32;
const VTOR_ADDRESS : *mut u32 = 0xE000_ED08 as *mut u32;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Starting Bootloader");

    defmt::println!("Reading from App Space");
    

    let stack_pointer = reg::reg_read(SP_ADDRESS);
    let program_counter = reg::reg_read(PC_ADDRESS);

    defmt::println!("Stack Pointer:{}", stack_pointer);
    defmt::println!("Program Counter:{}", program_counter);

    reg::reg_write(VTOR_ADDRESS, 0x0800_8000); //set VTOR to the address of the application

    gpio::a_init(); //gpio needs to be init before uart
    uart::uart_init();

    uart::uart_send_str("Bootloader ready\r\n> ");

    loop
    {
        let byte = uart::uart_receive();
        if byte == b'\r' {
            uart::uart_send_str("\r\n> ");
        } else {
            uart::uart_send(byte.to_ascii_uppercase());
        }
    }

    unsafe {
        asm!(
            "msr msp, {0}",
            "bx {1}",
            in(reg) stack_pointer,
            in(reg) program_counter,
            options(noreturn)
        );
    }

    bootloader::exit()
}
