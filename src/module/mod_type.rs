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
}
