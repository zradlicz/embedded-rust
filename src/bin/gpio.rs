use crate::reg;

const RCC_AHB2ENR : *mut u32 = 0x4002_104C as *mut u32;
const GPIOA_MODER : *mut u32 = 0x4800_0000 as *mut u32;
const GPIOA_OTYPER : *mut u32 = 0x4800_0004 as *mut u32;
const GPIOA_PUPDR : *mut u32 = 0x4800_000C as *mut u32;
const GPIOA_ODR : *mut u32 = 0x4800_0014 as *mut u32;
const GPIOA_BSRR : *mut u32 = 0x4800_0018 as *mut u32;

pub fn a_init()
{
    reg::reg_set_bit(RCC_AHB2ENR, 0); //activate clock for gpioA
    reg::reg_set_bit(GPIOA_MODER, 10); //set 5 to output
    reg::reg_clear_bit(GPIOA_MODER, 11);
}

pub fn a_set_pin(pin:u32)
{
    reg::reg_set_bit(GPIOA_BSRR, pin);
}

pub fn a_reset_pin(pin:u32)
{
    reg::reg_set_bit(GPIOA_BSRR, (pin) + 16);
}

pub fn a_toggle_pin(pin:u32)
{
    let reg_val = reg::reg_read(GPIOA_ODR);
    if reg_val & (1 << pin) != 0 {
        a_reset_pin(pin);
    } else {
        a_set_pin(pin);
    }
}