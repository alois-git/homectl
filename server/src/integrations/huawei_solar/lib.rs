use byteorder::{BigEndian, WriteBytesExt};
use modbus::Client;
use std::marker::PhantomData;
use std::str;

pub trait NumericRegisterTrait {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error>;
}

pub struct NumericRegister<T> {
    pub(crate) addr: u16,
    pub(crate) count: u16,
    pub(crate) gain: u32,
    pub(crate) unit: &'static str,
    pub(crate) marker: PhantomData<T>
}

impl<T> NumericRegister<T> {
    pub fn new(addr: u16, count: u16, gain: u32, unit: &'static str) -> NumericRegister<T> {
        NumericRegister {
            addr: addr,
            count: count,
            gain: gain,
            unit: unit,
            marker: PhantomData
        }
    }

    pub fn get_unit(&self) -> &'static str {
        self.unit
    }
}

impl NumericRegisterTrait for NumericRegister<u16> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut result: Vec<f64> = Vec::new();
        for elem in resp {
            result.push((elem as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

impl NumericRegisterTrait for NumericRegister<u32> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut bytes: Vec<u8> = Vec::new();
        for elem in resp {
            bytes.write_u16::<BigEndian>(elem).unwrap();
        }
        let mut result: Vec<f64> = Vec::new();
        for i in 0..(bytes.len() / 4) {
            result.push((u32::from_be_bytes([bytes[i * 4], bytes[i * 4 + 1], bytes[i * 4 + 2], bytes[i * 4 + 3]]) as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

impl NumericRegisterTrait for NumericRegister<i16> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut result: Vec<f64> = Vec::new();
        for elem in resp {
            result.push((elem as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

impl NumericRegisterTrait for NumericRegister<i32> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut bytes: Vec<u8> = Vec::new();
        for elem in resp {
            bytes.write_u16::<BigEndian>(elem).unwrap();
        }
        let mut result: Vec<f64> = Vec::new();
        for i in 0..(bytes.len() / 4) {
            result.push((i32::from_be_bytes([bytes[i * 4], bytes[i * 4 + 1], bytes[i * 4 + 2], bytes[i * 4 + 3]]) as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

pub struct StringRegister {
    pub(crate) addr: u16,
    pub(crate) count: u16
}

impl StringRegister {
    pub fn new(addr: u16, count: u16) -> StringRegister {
        StringRegister {
            addr: addr,
            count: count
        }
    }

    pub fn read(&self, client: &mut modbus::tcp::Transport) -> Result<String, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut bytes: Vec<u8> = Vec::new();
        for elem in resp {
            bytes.write_u16::<BigEndian>(elem).unwrap();
        }
        Ok(str::from_utf8(&bytes).unwrap().replace(char::from(0), "").to_string())
    }
}





