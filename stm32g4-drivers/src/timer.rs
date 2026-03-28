use crate::systick;

pub struct Timer
{
    start_tick: u32,
    duration_ticks : u32,
    is_active : bool,
    is_periodic : bool,
    callback : fn(),
}

impl Timer
{
    pub fn new(duration_ticks: u32, is_periodic: bool, callback: fn()) -> Self
    {
        Timer
        {
            start_tick : 0,
            duration_ticks,
            is_active : false,
            is_periodic,
            callback,
        }
    }

    pub fn start(&mut self)
    {
        self.start_tick = systick::systick_get_tick();
        self.is_active = true;
    }

    pub fn stop(&mut self)
    {
        self.is_active = false;
    }

    pub fn check(&mut self)
    {
        if !self.is_active { return; }

        if self.start_tick.wrapping_sub(systick::systick_get_tick()) >= self.duration_ticks
        {
            (self.callback)();

            if self.is_periodic
            {
                self.start_tick = systick::systick_get_tick();
            }
            else
            {
                self.is_active = false;
            }
        }
    }
}
