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
    pub fn as_str(&self) -> &str {
        match *self {
            ModuleType::Gate => "gate",
            ModuleType::Servo => "servo",
            ModuleType::RgbLed => "led",
            ModuleType::Potentiometer => "potard",
            ModuleType::Button => "button",
            ModuleType::DistanceSensor => "distance",
            ModuleType::Relay => "relay",
            ModuleType::DynamixelMotor => "dynamixel",
            ModuleType::Stepper => "stepper",
            ModuleType::Encoder => "encoder",
            ModuleType::InputGPIO => "gpio",
            _ => panic!("unsupported module type!"),
        }
    }
    pub fn as_field(&self) -> &str {
        match *self {
            ModuleType::DistanceSensor => "distance",
            ModuleType::Button | ModuleType::InputGPIO => "state",
            ModuleType::Potentiometer
            | ModuleType::Encoder
            | ModuleType::DynamixelMotor
            | ModuleType::HomeMadeServo
            | ModuleType::Stepper => "position",
            _ => panic!("unsupported module type!"),
        }
    }
}
