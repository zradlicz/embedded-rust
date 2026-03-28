use crate::reg;

const RCC_APB1ENR2  : *mut u32 = 0x4002_105C as *mut u32;
const LPUART1_CR1   : *mut u32 = 0x4000_8000 as *mut u32;
const LPUART1_CR2   : *mut u32 = 0x4000_8004 as *mut u32;
const LPUART1_BRR   : *mut u32 = 0x4000_800C as *mut u32;
const LPUART1_ISR   : *mut u32 = 0x4000_801C as *mut u32;
const LPUART1_RDR   : *mut u32 = 0x4000_8024 as *mut u32;
const LPUART1_TDR   : *mut u32 = 0x4000_8028 as *mut u32;

// LPUART BRR = (256 * fCLK) / baud = (256 * 16MHz) / 115200 = 35556
const BRR_115200    : u32 = 35556;

const UE  : u32 = 0;
const TE  : u32 = 3;
const RE  : u32 = 2;

const TXE  : u32 = 7;
const TC   : u32 = 6;
const RXNE : u32 = 5;

pub fn uart_init()
{
    //start clock for uart
    reg::reg_set_bit(RCC_APB1ENR2, 0);

    //program the m bits in USART_CR1 to define word length (8 bits, default)
    //program stop bits in USART_CR2 (1 stop bit, default)

    //set baud rate in USART_BRR
    reg::reg_write(LPUART1_BRR, BRR_115200);

    //set the TE bit in USART_CR1 to send idle frame as first transmission
    reg::reg_set_bit(LPUART1_CR1, TE);
    reg::reg_set_bit(LPUART1_CR1, RE);

    //enable USART by writing UE bit in USART_CR1
    reg::reg_set_bit(LPUART1_CR1, UE);
}

pub fn uart_send(data: u8)
{
    //write the data in the USART_TDR register (wait for TXE first)
    while reg::reg_read(LPUART1_ISR) & (1 << TXE) == 0 {}

    //repeat for each data
    reg::reg_write(LPUART1_TDR, data as u32);

    //when last frame is written, wait for TC=1 in the USART_ISR reg
    while reg::reg_read(LPUART1_ISR) & (1 << TC) == 0 {}
}

pub fn uart_send_str(s: &str)
{
    for byte in s.bytes() {
        uart_send(byte);
    }
}

pub fn uart_receive() -> u8
{
    //program the M bit in LPUART_CR1 to define word length (8 bits, default)
    //set baud rate using LPUART_BRR (done in uart_init)
    //program stop bits in LPUART_CR2 (1 stop bit, default)
    //enable by writing UE bit on LPUART_CR1 (done in uart_init)
    //set RE bit in LPUART_CR1 (done in uart_init)

    //when FIFO mode is disabled, the RXNE bit is set when data has been received and can be read
    while reg::reg_read(LPUART1_ISR) & (1 << RXNE) == 0 {}

    //read received byte from RDR
    reg::reg_read(LPUART1_RDR) as u8
}
