use rppal::i2c::I2c;


#[derive(Clone, Copy)]
struct PWMChannel {
    frequency: u16,
}

impl PWMChannel {
    pub fn get_frequency(&self)-> u16 {
        self.frequency
    }
}

pub struct PCA9685 {
    channels: [Option<PWMChannel>; 16],
    i2c_device: I2c,
    reference_clock_speed: u32,
    prescale_reg: u32,
    model_reg: u32,
}

impl PCA9685 {
    pub fn new(mut i2c_device: I2c, address: u16, reference_clock_speed: u32) -> Self {
        i2c_device.set_slave_address(address).unwrap();

        Self {
            i2c_device,
            reference_clock_speed,
            channels: [None; 16],
            prescale_reg: 0xFE,
            model_reg: 0x00,
        }
    }

    pub fn get_frequency(&self) -> u32 {
        self.reference_clock_speed / 4096 / self.prescale_reg
    }

    pub fn set_frequency(&self) {
        
    }
}

impl Default for PCA9685 {
    fn default() -> Self {
        Self::new(I2c::new().unwrap(), 0x40, 25_000_000)
    }
}