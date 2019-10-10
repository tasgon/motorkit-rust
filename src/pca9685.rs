use rppal::i2c::I2c;


// The following constants taken from https://github.com/adafruit/Adafruit_Python_PCA9685/blob/master/Adafruit_PCA9685/PCA9685.py
// Registers
const PCA9685_ADDRESS: u16    = 0x40;
const MODE1: u16              = 0x00;
const MODE2: u16              = 0x01;
const SUBADR1: u16            = 0x02;
const SUBADR2: u16            = 0x03;
const SUBADR3: u16            = 0x04;
const PRESCALE: u16           = 0xFE;
const LED0_ON_L: u16          = 0x06;
const LED0_ON_H: u16          = 0x07;
const LED0_OFF_L: u16         = 0x08;
const LED0_OFF_H: u16         = 0x09;
const ALL_LED_ON_L: u16       = 0xFA;
const ALL_LED_ON_H: u16       = 0xFB;
const ALL_LED_OFF_L: u16      = 0xFC;
const ALL_LED_OFF_H: u16      = 0xFD;

// Bits
const RESTART: u16            = 0x80;
const SLEEP: u16              = 0x10;
const ALLCALL: u16            = 0x01;
const INVRT: u16              = 0x10;
const OUTDRV: u16             = 0x04;
const SWRESET: u16            = 0x06;

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