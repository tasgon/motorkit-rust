use pwm_pca9685::Pca9685;

#[derive(Copy, Clone)]
struct Motor<'a> {
    positive_pwm: u16,
    negative_pwm: u16,
    pca: &'a Pca9685,
}

impl<'a> Motor<'a> {
    pub fn new<'b>(positive_pwm: u16, negative_pwm: u16, pca: &'a Pca9685) -> Self<'a> {
        Self {
            positive_pwm,
            negative_pwm,
            pca,
        }
    } 
}