/// Internal Protocol Command
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProtocolCommand {
    GetId,
    WriteId,
    WriteAlias,
    GetModuleType,
    GetStatus,
    GetFirmRevision,
    GetComRevision,
    _OffsetNumber = 30,
}

/// Available Command for `Message`
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    /// Gate asks a module to identify itself
    Identify = ProtocolCommand::_OffsetNumber as isize,
    /// Module sends its alias and type to the gate
    Introduction,
    /// Gate asks a sensor module to publish its data
    GetState,
    /// Module publishes its data to the gate
    PublishState,

    /// Module Specific register

    /// Led Module
    /// Led color - size = 3 (R, G, B)
    LedColor,

    /// Servo Module
    /// Servo position - size = 1 (degree)
    ServoPosition,
    /// Servo speed - size 1 (degree/s)
    ServoSpeed,
    /// Set servo wheel mode - size 1 (True/False)
    WheelMode,
    /// Set servo compliant - size 1 (True/False)
    SetCompliant,

    /// Relay Module
    /// Enable relay - size = 1 (True/False)
    EnableRelay,

    /// Stepper Module
    /// Get stepper position - size = 1 (steps)
    StepperPosition,
    /// Get stepper speed - size = 1 (steps/s)
    StepperSpeed,
    /// Set stepper to home position - size = 1 (True/False)
    StepperHomePosition,
    /// Stop stepper
    StepperStop,

    /// Misc
    LedPower, // size = 1 (brightness)
    SetAsservStep, // P (2 bytes), I (2 bytes), D (2 bytes), target (1 bytes)
    GetAsservStep,
    EncoderHome,
    PowerRatio,

    _GateProtocolOffsetNumber,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command_offset() {
        assert_eq!(
            Command::Identify as u8,
            ProtocolCommand::_OffsetNumber as u8
        );
    }
}
