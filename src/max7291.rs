use embassy_stm32::{gpio::Output, mode::Blocking, spi::{Spi, mode::Master}};

pub struct Max7291{
    spi: Spi<'static, Blocking, Master>,
    cs: Output<'static>,
    curr_display: [u8; 8],
}

impl Max7291{
    pub fn new(spi: Spi<'static, Blocking, Master>, cs: Output<'static>) -> Self {
        Self {
            spi, 
            cs,
            curr_display: [0u8; 8],
        }
    }
    pub fn init(&mut self) -> Result<(), embassy_stm32::spi::Error> {
        self.send(0x0F, 0x00)?;  // display test off
        self.send(0x0C, 0x01)?;  // exit shutdown
        self.send(0x0B, 0x07)?;  // scan limit = all 8 rows
        self.send(0x09, 0x00)?;  // no decode mode (raw LED control)
        self.send(0x0A, 0x08)?;
        Ok(())
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, on: bool) {
        // curr_display[row] holds the LED state for that row.
        // Each bit = one column.
        // curr_display[3] = 0b00010000 means row 3, column 4 is on, rest off.
        if on {
            self.curr_display[row] |= 1 << (7-col);
        } else {
            self.curr_display[row] &= !(1 << (7-col));
        }
    }

    pub fn flush_display(&mut self) -> Result<(), embassy_stm32::spi::Error>{
        let curr = self.curr_display;
        for (i, &row) in curr.iter().enumerate() {
            self.send((i + 1) as u8, row)?;
        }
        Ok(())
    }

    fn send(&mut self, reg: u8, data: u8) -> Result<(), embassy_stm32::spi::Error> {
        let cmd = [reg, data];
        self.cs.set_low();
        self.spi.blocking_write(&cmd)?;
        self.cs.set_high();
        Ok(())
    }
}
