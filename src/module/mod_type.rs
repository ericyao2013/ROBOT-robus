/// Available `Module` type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModuleType {
    Gate,
    Servo,
    RgbLed,
    Potentiometer,
    Button,
    DistanceSensor,
    Relay,
    DynamixelMotor,
    Stepper,
    HomeMadeServo,
    Ledstrip,
    Rtc,
    Encoder,
    GenericMotor,
    Sniffer,
    InputGPIO,
}
impl ModuleType {
    pub fn is_sensor(&self) -> bool {
        match *self {
            ModuleType::Potentiometer
            | ModuleType::Button
            | ModuleType::DistanceSensor
            | ModuleType::DynamixelMotor
            | ModuleType::Encoder
            | ModuleType::InputGPIO => true,
            _ => false,
        }
    }
}
