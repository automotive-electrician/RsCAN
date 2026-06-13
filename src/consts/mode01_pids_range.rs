// PIDs Supported (01-20) -> 0x00 -> Bitcode
pub const MIN_0X00: u32 = 0x00;
pub const MAX_0X00: u32 = 0xFFFFFFFF;

// Monitor Status Since DTCs Cleared -> 0x01 -> 4Byte
pub const MIN_0X01: u32 = 0x00;
pub const MAX_0X01: u32 = 0xFFFFFFFF;

// Freeze DTC -> 0x02

// Fuel System Status -> 0x03
pub const MIN_0X03: u8 = 0;
pub const MAX_0X03: u8 = 255;

// Calculated Engine Load -> 0x04
pub const MIN_0X04: f32 = 0.00;
pub const MAX_0X04: f32 = 100.00;

// Engine Coolant Temperature -> 0x05
pub const MIN_0X05: i16 = -40;
pub const MAX_0X05: i16 = 215;

// Short and Long Term Fuel Trim - Bank -> 0x06, 0x07, 0x08, 0x09
pub const MIN_0X06_0X07_0X08_0X09: f32 = -100.00000;
pub const MAX_0X06_0X07_0X08_0X09: f32 = 99.21875;

// Fuel Pressure -> 0x0A
pub const MIN_0X0A: u16 = 0;
pub const MAX_0X0A: u16 = 765;

// Intake Manifold Absolute Pressure -> 0x0B
pub const MIN_0X0B: u8 = 0;
pub const MAX_0X0B: u8 = 255;

// Engine RPM -> 0x0C
pub const MIN_0X0C: f32 = 0.00;
pub const MAX_0X0C: f32 = 16383.75;

// Vehicle Speed -> 0x0D
pub const MIN_0X0D: u8 = 0;
pub const MAX_0X0D: u8 = 255;

// Timing Advance -> 0x0E
pub const MIN_0X0E: f32 = -64.0;
pub const MAX_0X0E: f32 = 63.5;

// Intake Air Temperature -> 0x0F
pub const MIN_0X0F: i16 = -40;
pub const MAX_0X0F: i16 = 215;

// MAF Air Flow Rate -> 0x10
pub const MIN_0X10: f32 = 0.00;
pub const MAX_0X10: f32 = 655.35;

// Throttle Position -> 0x11
pub const MIN_0X11: f32 = 0.00;
pub const MAX_0X11: f32 = 100.00;

// Commanded Secondary Air Status -> 0x12 -> Bitmap

// Oxygen Sensors Present (2 Banks) -> 0x13 -> Bitfield

// Oxygen Sensors Voltage -> 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B
pub const MIN_0X14_0X15_0X16_0X17_0X18_0X19_0X1A_0X1B_VOLTAGE: f32 = 0.00;
pub const MAX_0X14_0X15_0X16_0X17_0X18_0X19_0X1A_0X1B_VOLTAGE: f32 = 1.275;

// Oxygen Sensors Fuel Trim -> 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B
pub const MIN_0X14_0X15_0X16_0X17_0X18_0X19_0X1A_0X1B_FUEL_TRIM: f32 = -100.00;
pub const MAX_0X14_0X15_0X16_0X17_0X18_0X19_0X1A_0X1B_FUEL_TRIM: f32 = 99.21875;

// OBD Standard Compliance -> 0x1C
pub const MIN_0X1C: u8 = 1;
pub const MAX_0X1C: u8 = 255;

// Oxygen Sensors Present (4 Banks) -> 0x1D -> Bitfield

// AUX Input Status -> 0x1E -> bit-encoded

// Run time Since Engine Start -> 0x1F
pub const MIN_0X1F: u16 = 0;
pub const MAX_0X1F: u16 = 65535;

// PIDs Supported (21..40) -> 0x20 -> Bitfield

// Distance Traveled with MIL On -> 0x21
pub const MIN_0X21: u16 = 0;
pub const MAX_0X21: u16 = 65535;

// Fuel Rail Gauge Pressure (Relative) -> 0x22
pub const MIN_0X22: f32 = 0.00;
pub const MAX_0X22: f32 = 5177.265;

// Fuel Rail Gauge Pressure -> 0x23
pub const MIN_0X23: u32 = 0;
pub const MAX_0X23: u32 = 655350;

// Oxygen Sensors (Lambda Voltage) -> 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B
pub const MIN_0X24_0X25_0X26_0X27_0X28_0X29_0X2A_0X2B: f32 = 0.00;
pub const MAX_0X24_0X25_0X26_0X27_0X28_0X29_0X2A_0X2B: f32 = 7.999;

// Commanded EGR -> 0x2C
pub const MIN_0X2C: f32 = 0.00;
pub const MAX_0X2C: f32 = 100.00;

// EGR Error -> 0x2D
pub const MIN_0X2D: f32 = -100.00;
pub const MAX_0X2D: f32 = 99.22;

// Commanded Evaporative Purge -> 0x2E
pub const MIN_0X2E: f32 = 0.00;
pub const MAX_0X2E: f32 = 100.00;

// Fuel Tank Level Input -> 0x2F
pub const MIN_0X2F: f32 = 0.00;
pub const MAX_0X2F: f32 = 100.00;

// Warm-ups Since Codes Cleared -> 0x30
pub const MIN_0X30: u8 = 0;
pub const MAX_0X30: u8 = 255;

// Distance Traveled Since Codes Cleared -> 0x31
pub const MIN_0X31: u16 = 0;
pub const MAX_0X31: u16 = 65535;

// EVAP System Vapor Pressure -> 0x32
pub const MIN_0X32: f32 = -8192.00;
pub const MAX_0X32: f32 = 8191.75;

// Absolute Barometric Pressure -> 0x33
pub const MIN_0X33: u8 = 0;
pub const MAX_0X33: u8 = 255;

// Oxygen Sensors (Lambda Current) -> 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B
pub const MIN_0X34_0X35_0X36_0X37_0X38_0X39_0X3A_0X3B: f32 = -128.00;
pub const MAX_0X34_0X35_0X36_0X37_0X38_0X39_0X3A_0X3B: f32 = 127.996;

// Catalyst Temp -> 0x3C, 0x3D, 0x3E, 0x3F
pub const MIN_0X3C_0X3D_0X3E_0X3F: f32 = -40.00;
pub const MAX_0X3C_0X3D_0X3E_0X3F: f32 = 6513.5;

// PIDs Supported (41..60) -> 0x40 -> bitcode

// Monitor Status This Drive Cycle -> 0x41 -> 4 byte

// Control Module Voltage -> 0x42
pub const MIN_0X42: f32 = 0.00;
pub const MAX_0X42: f32 = 65.535;

// Absolute Load Value -> 0x43
pub const MIN_0X43: f32 = 0.00;
pub const MAX_0X43: f32 = 25700.00;

// Commanded Lambda -> 0x44
pub const MIN_0X44: f32 = 0.00;
pub const MAX_0X44: f32 = 1.999;

// Relative Throttle Position -> 0x45
pub const MIN_0X45: f32 = 0.00;
pub const MAX_0X45: f32 = 100.00;

// Ambient Air Temperature -> 0x46
pub const MIN_0X46: i16 = -40;
pub const MAX_0X46: i16 = 215;

// Absolute Throttle and Accelerator Pedal Positions -> 0x47, 0x48, 0x49, 0x4A, 0x4B
pub const MIN_0X47_0X48_0X49_0X4A_0X4B: f32 = 0.00;
pub const MAX_0X47_0X48_0X49_0X4A_0X4B: f32 = 100.00;

// Commanded Throttle Actuator -> 0x4C
pub const MIN_0X4C: f32 = 0.00;
pub const MAX_0X4C: f32 = 100.00;

// Time Run with MIL On -> 0x4D
pub const MIN_0X4D: u16 = 0;
pub const MAX_0X4D: u16 = 65535;

// Time Since Trouble Codes Cleared -> 0x4E
pub const MIN_0X4E: u16 = 0;
pub const MAX_0X4E: u16 = 65535;

// Max Values -> 0x4F -> 4 Byte

// Max MAF Air Flow Rate
pub const MIN_0X50: f32 = 0.00;
pub const MAX_0X50: f32 = 655.35;

// Fuel Type -> 0x51
pub const MIN_0X51: u8 = 1;
pub const MAX_0X51: u8 = 255;

// Ethanol Fuel Percentage -> 0x52
pub const MIN_0X52: f32 = 0.00;
pub const MAX_0X52: f32 = 100.00;

// Absolute EVAP System Vapor Pressure -> 0x53
pub const MIN_0X53: f32 = 0.00;
pub const MAX_0X53: f32 = 327.675;

// EVAP System Vapor Pressure (Pa) -> 0x54
pub const MIN_0X54: f32 = 0.00;
pub const MAX_0X54: f32 = 16383.75;

// Short and Long Term Secondary O2 Trims -> 0x55, 0x56, 0x57, 0x58
pub const MIN_0X55_0X56_0X57_0X58: f32 = -100.00;
pub const MAX_0X55_0X56_0X57_0X58: f32 = 99.22;

// Fuel Rail Absolute Pressure -> 0x59
pub const MIN_0X59: u32 = 0;
pub const MAX_0X59: u32 = 655350;

// Relative Accelerator Pedal Position -> 0x5A
pub const MIN_0X5A: f32 = 0.00;
pub const MAX_0X5A: f32 = 100.00;

// Hybrid Battery Pack Life -> 0x5B
pub const MIN_0X5B: f32 = 0.00;
pub const MAX_0X5B: f32 = 100.00;

// Engine Oil Temperature -> 0x5C
pub const MIN_0X5C: i16 = -40;
pub const MAX_0X5C: i16 = 215;

// Fuel Injection Timing -> 0x5D
pub const MIN_0X5D: f32 = -210.000;
pub const MAX_0X5D: f32 = 301.992;

// Engine Fuel Rate -> 0x5E
pub const MIN_0X5E: f32 = 0.00;
pub const MAX_0X5E: f32 = 3276.75;

// Emission Requirements -> 0x5F -> Bitcode

// PIDs Supported (61-80) -> 0x60

// Driver's Demand Engine Torque -> 0x61
pub const MIN_0X61: i16 = -125;
pub const MAX_0X61: i16 = 130;

// Actual Engine Torque -> 0x62
pub const MIN_0X62: i16 = -125;
pub const MAX_0X62: i16 = 130;

// Engine Reference Torque -> 0x63
pub const MIN_0X63: u16 = 0;
pub const MAX_0X63: u16 = 65535;

// Aux Input/Output Support -> 0x64 -> Bitcode

// Mass Air Flow Sensor 2 -> 0x65
pub const MIN_0X65: f32 = 0.00;
pub const MAX_0X65: f32 = 655.35;

// Engine Coolant Temperature 2 -> 0x66
pub const MIN_0X66: i16 = -40;
pub const MAX_0X66: i16 = 215;

// Intake Air Temperature 2 -> 0x67
pub const MIN_0X67: i16 = -40;
pub const MAX_0X67: i16 = 215;
