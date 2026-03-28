pub fn reg_read(reg:*mut u32) -> u32
{
    unsafe { core::ptr::read_volatile(reg) }
}

pub fn reg_write(reg:*mut u32, val: u32)
{
    unsafe { core::ptr::write_volatile(reg, val) }
}

pub fn reg_set_bit(reg:*mut u32, bit:u32)
{
    let val = reg_read(reg);
    reg_write(reg, val | (1 << bit));
}

pub fn reg_clear_bit(reg:*mut u32, bit:u32)
{
    let val = reg_read(reg);
    reg_write(reg, val & !(1 << bit));
}
