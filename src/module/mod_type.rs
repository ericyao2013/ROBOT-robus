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
    GenericIO,
    L0GPIO,
    L0Servo,
    L0DCmotor,
}
impl ModuleType {
    pub fn is_sensor(&self) -> bool {
        match *self {
            ModuleType::Potentiometer
            | ModuleType::Button
            | ModuleType::DistanceSensor
            | ModuleType::DynamixelMotor
            | ModuleType::Encoder
            | ModuleType::GenericIO
            | ModuleType::L0GPIO => true,
            _ => false,
        }
    }
    pub fn as_str(&self) -> &str {
        match *self {
            ModuleType::Gate => "gate",
            ModuleType::Servo => "servo",
            ModuleType::RgbLed => "led",
            ModuleType::Ledstrip => "led_strip",
            ModuleType::Potentiometer => "potard",
            ModuleType::Button => "button",
            ModuleType::DistanceSensor => "distance",
            ModuleType::Relay => "relay",
            ModuleType::DynamixelMotor => "dynamixel",
            ModuleType::Stepper => "stepper",
            ModuleType::Encoder => "encoder",
            ModuleType::Rtc => "rtc",
            ModuleType::Sniffer => "sniffer",
            ModuleType::GenericMotor => "generic_motor",
            ModuleType::HomeMadeServo => "home_made_servo",
            ModuleType::GenericIO => "GenericIO",
            ModuleType::L0GPIO => "l0_gpio",
            ModuleType::L0Servo => "l0_servo",
            ModuleType::L0DCmotor => "l0_dc_motor",
        }
    }
    pub fn as_field(&self) -> &str {
        match *self {
            ModuleType::DistanceSensor => "distance",
            ModuleType::Button
            | ModuleType::GenericIO
            | ModuleType::L0GPIO
            | ModuleType::L0Servo => "state",
            ModuleType::Potentiometer
            | ModuleType::Encoder
            | ModuleType::DynamixelMotor
            | ModuleType::HomeMadeServo
            | ModuleType::Stepper => "position",
            _ => panic!("unsupported module type!"),
        }
    }
}
