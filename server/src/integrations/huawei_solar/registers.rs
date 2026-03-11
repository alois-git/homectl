use std::marker::PhantomData;
use crate::integrations::huawei_solar::lib::{NumericRegister, StringRegister};

pub enum Register {
    I16(&'static str, &'static NumericRegister<i16>),
    I32(&'static str, &'static NumericRegister<i32>),
}

pub const METER_STATUS: NumericRegister<i16> = NumericRegister::<i16> { addr: 37100, count: 1, gain: 1, unit: "watt", marker: PhantomData };
pub const METER_ACTIVE_POWER_L1: NumericRegister<i32> = NumericRegister::<i32> { addr: 37101, count: 2, gain: 1, unit: "watt", marker: PhantomData };
pub const METER_ACTIVE_POWER_L2: NumericRegister<i32> = NumericRegister::<i32> { addr: 37103, count: 2, gain: 0, unit: "watt", marker: PhantomData };
pub const METER_ACTIVE_POWER_L3: NumericRegister<i32> = NumericRegister::<i32> { addr: 37105, count: 2, gain: 0, unit: "watt", marker: PhantomData };

pub const METER_VOLTAGE_L1: NumericRegister<i32> = NumericRegister::<i32> { addr: 37107, count: 2, gain: 10, unit: "volt", marker: PhantomData };
pub const METER_VOLTAGE_L2: NumericRegister<i32> = NumericRegister::<i32> { addr: 37109, count: 2, gain: 10, unit: "volt", marker: PhantomData };
pub const METER_VOLTAGE_L3: NumericRegister<i32> = NumericRegister::<i32> { addr: 37111, count: 2, gain: 10, unit: "volt", marker: PhantomData };

pub const METER_CURRENT_L1: NumericRegister<i32> = NumericRegister::<i32> { addr: 37113, count: 2, gain: 1000, unit: "ampere", marker: PhantomData };
pub const METER_CURRENT_L2: NumericRegister<i32> = NumericRegister::<i32> { addr: 37115, count: 2, gain: 1000, unit: "ampere", marker: PhantomData };
pub const METER_CURRENT_L3: NumericRegister<i32> = NumericRegister::<i32> { addr: 37117, count: 2, gain: 1000, unit: "ampere", marker: PhantomData };

pub const METER_TOTAL_ACTIVE_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 37119, count: 2, gain: 1, unit: "watt", marker: PhantomData };

pub const METER_POWER_FACTOR_L1: NumericRegister<i16> = NumericRegister::<i16> { addr: 37121, count: 1, gain: 1000, unit: "", marker: PhantomData };
pub const METER_POWER_FACTOR_L2: NumericRegister<i16> = NumericRegister::<i16> { addr: 37123, count: 1, gain: 1000, unit: "", marker: PhantomData };
pub const METER_POWER_FACTOR_L3: NumericRegister<i16> = NumericRegister::<i16> { addr: 37125, count: 1, gain: 1000, unit: "", marker: PhantomData };

pub const METER_GRID_FREQUENCY: NumericRegister<u16> = NumericRegister::<u16> { addr: 37127, count: 1, gain: 100, unit: "Hz", marker: PhantomData };

pub const METER_IMPORT_ACTIVE_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 37129, count: 2, gain: 100, unit: "kWh", marker: PhantomData };
pub const METER_EXPORT_ACTIVE_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 37131, count: 2, gain: 100, unit: "kWh", marker: PhantomData };
pub const METER_TOTAL_REACTIVE_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 37133, count: 2, gain: 1, unit: "var", marker: PhantomData };

pub const METER_IMPORT_REACTIVE_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 37135, count: 2, gain: 100, unit: "kvarh", marker: PhantomData };
pub const METER_EXPORT_REACTIVE_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 37137, count: 2, gain: 100, unit: "kvarh", marker: PhantomData };

pub const MODEL_NAME: StringRegister = StringRegister { addr: 30000, count: 15 };
pub const SERIAL_NUMBER: StringRegister = StringRegister { addr: 30015, count: 10 };
pub const INPUT_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 32064, count: 2, gain: 1, unit: "W", marker: PhantomData };
pub const GRID_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32066, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const LINE_VOLTAGE_A_B: NumericRegister<u16> = NumericRegister::<u16> { addr: 32066, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const LINE_VOLTAGE_B_C: NumericRegister<u16> = NumericRegister::<u16> { addr: 32067, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const LINE_VOLTAGE_C_A: NumericRegister<u16> = NumericRegister::<u16> { addr: 32068, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const PHASE_A_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32069, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const PHASE_B_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32070, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const PHASE_C_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32071, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const GRID_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32072, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const PHASE_A_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32072, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const PHASE_B_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32074, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const PHASE_C_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32076, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const DAY_ACTIVE_POWER_PEAK: NumericRegister<i32> = NumericRegister::<i32> { addr: 32078, count: 2, gain: 1, unit: "W", marker: PhantomData };
pub const ACTIVE_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 32080, count: 2, gain: 1, unit: "W", marker: PhantomData };
pub const REACTIVE_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 32082, count: 2, gain: 1, unit: "VA", marker: PhantomData };
pub const POWER_FACTOR: NumericRegister<i16> = NumericRegister::<i16> { addr: 32084, count: 1, gain: 1000, unit: "", marker: PhantomData };
pub const GRID_FREQUENCY: NumericRegister<u16> = NumericRegister::<u16> { addr: 32085, count: 1, gain: 100, unit: "Hz", marker: PhantomData };
pub const EFFICIENCY: NumericRegister<u16> = NumericRegister::<u16> { addr: 32086, count: 1, gain: 100, unit: "%", marker: PhantomData };
pub const INTERNAL_TEMPERATURE: NumericRegister<i16> = NumericRegister::<i16> { addr: 32087, count: 1, gain: 10, unit: "°C", marker: PhantomData };
pub const INSULATION_RESISTANCE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32088, count: 1, gain: 100, unit: "MOhm", marker: PhantomData };
pub const DEVICE_STATUS: NumericRegister<u16> = NumericRegister::<u16> { addr: 32089, count: 1, gain: 1, unit: "", marker: PhantomData };
// FAULT_CODE
// TODO: timestamps
pub const STARTUP_TIME: NumericRegister<u32> = NumericRegister::<u32> { addr: 32091, count: 2, gain: 1, unit: "s", marker: PhantomData };
pub const SHUTDOWN_TIME: NumericRegister<u32> = NumericRegister::<u32> { addr: 32093, count: 2, gain: 1, unit: "s", marker: PhantomData };
pub const ACCUMULATED_YIELD_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 32106, count: 2, gain: 100, unit: "kWh", marker: PhantomData };
pub const DAILY_YIELD_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 32114, count: 2, gain: 100, unit: "kWh", marker: PhantomData };
pub const TIME_ZONE: NumericRegister<i16> = NumericRegister::<i16> { addr: 43006, count: 1, gain: 1, unit: "min", marker: PhantomData };