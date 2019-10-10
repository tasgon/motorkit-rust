use pwm_pca9685::Pca9685;
use linux_embedded_hal::I2cdev;

pub mod motor;

pub struct MotorKit {
    motors: [Option<Motor>; 4],
    pca: Pca9685,
}

impl MotorKit {
    pub fn new(address: u16, i2c: I2cdev, steppers_microsteps: u16) -> Self {
        Self {
            motors: [None; 4],
            pca: Pca9685::new(i2c, address),
        }
    }
}

impl Default for MotorKit {
    pub fn default() -> Self {
        Self::new(0x60, I2cdev::new("/dev/i2c-1"), 16)
    }

    fn motor(id: u8, c1: u16, c2: u16, c3: u16) -> Motor {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
