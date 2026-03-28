use crate::reg;

const RCC_AHB2ENR : *mut u32 = 0x4002_104C as *mut u32;
const GPIOA_MODER : *mut u32 = 0x4800_0000 as *mut u32;
const GPIOA_OTYPER : *mut u32 = 0x4800_0004 as *mut u32;
const GPIOA_PUPDR : *mut u32 = 0x4800_000C as *mut u32;
const GPIOA_ODR : *mut u32 = 0x4800_0014 as *mut u32;
const GPIOA_BSRR : *mut u32 = 0x4800_0018 as *mut u32;
const GPIOA_AFRL : *mut u32 = 0x4800_0020 as *mut u32;

pub fn a_init()
{
    reg::reg_set_bit(RCC_AHB2ENR, 0); // enable GPIOA clock

    // PA5: output for LED (MODER bits 11:10 = 01)
    reg::reg_set_bit(GPIOA_MODER, 10);
    reg::reg_clear_bit(GPIOA_MODER, 11);

    // PA2 (TX): alternate function (MODER bits 5:4 = 10)
    reg::reg_set_bit(GPIOA_MODER, 5);
    reg::reg_clear_bit(GPIOA_MODER, 4);

    // PA3 (RX): alternate function (MODER bits 7:6 = 10)
    reg::reg_set_bit(GPIOA_MODER, 7);
    reg::reg_clear_bit(GPIOA_MODER, 6);

    // PA2 AF12 (LPUART1): AFRL bits 11:8 = 1100
    reg::reg_clear_bit(GPIOA_AFRL, 8);
    reg::reg_clear_bit(GPIOA_AFRL, 9);
    reg::reg_set_bit(GPIOA_AFRL, 10);
    reg::reg_set_bit(GPIOA_AFRL, 11);

    // PA3 AF12 (LPUART1): AFRL bits 15:12 = 1100
    reg::reg_clear_bit(GPIOA_AFRL, 12);
    reg::reg_clear_bit(GPIOA_AFRL, 13);
    reg::reg_set_bit(GPIOA_AFRL, 14);
    reg::reg_set_bit(GPIOA_AFRL, 15);
}

pub fn a_set_pin(pin:u32)
{
    reg::reg_set_bit(GPIOA_BSRR, pin);
}

pub fn a_reset_pin(pin:u32)
{
    reg::reg_set_bit(GPIOA_BSRR, pin + 16);
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
