use crate::reg;

const SYST_CSR : *mut u32 = 0xE000_E010 as *mut u32;
const SYST_RVR : *mut u32 = 0xE000_E014 as *mut u32;
const SYST_CVR : *mut u32 = 0xE000_E018 as *mut u32;
const SYST_CALIB : *mut u32 = 0xE000_E01C as *mut u32;

pub fn systick_init()
{
    reg::reg_write(SYST_RVR, 0x00FF_FFFF);
    reg::reg_set_bit(SYST_CSR, 0);
}

pub fn systick_get_tick() -> u32
{
    reg::reg_read(SYST_CVR)
}