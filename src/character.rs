use embedded_hal::delay::DelayNs;

use crate::{bus::DataBus, charset::CharsetWithFallback, error::Result, memory_map::DisplayMemoryMap, HD44780};

impl<B, M, C> HD44780<B, M, C>
where
	B: DataBus,
	M: DisplayMemoryMap,
	C: CharsetWithFallback,
{
	pub fn define_custom_character<D: DelayNs>(
		&mut self,
		code: u8,
		def: &[u8; 8],
		delay: &mut D,
	) -> Result<(), B::Error> {
        self.write_command(0b01000000 | (code & 0x07) << 3, delay)?;
        delay.delay_us(100);

        let lines = def.iter().cloned();
        for line in lines {
            self.write_byte(line, delay)?;
        }
		Ok(())
	}
}