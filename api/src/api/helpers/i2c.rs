use std::collections::HashMap;
use std::io::{self, ErrorKind};
use i2cdev::core::*;
use i2cdev::linux::{
  LinuxI2CBus,
  LinuxI2CError,
  LinuxI2CMessage,
};
use crate::models::{NewDevices, Points};
use super::props::{
    I2C_LIGHT_LEVEL_START_OFFSET,
    I2C_RANGE_MIN,
    I2C_RANGE_MAX,
    I2C_BYTES_PER_LIGHT,
    LIGHT_LEVEL_MAX,
};

pub struct LightDevices {
    driver: Driver,
}

impl LightDevices {
    pub fn new(connection: u8) -> Result<Self, LinuxI2CError> {
        let driver = Driver::new(connection)?;
        Ok(Self { driver })
    }

    pub fn test(connection: u8) -> Result<(), LinuxI2CError> {
        Driver::new(connection)?;
        Ok(())
    }

    pub fn convert_points(points: Vec<Points>, override_active: bool) -> Vec<(i32, Vec<i32>)> {
        let mut mapped: HashMap<i32, Vec<Points>> = HashMap::new();
        for point in points {
            mapped.entry(point.device_id.clone())
                .and_modify(|v| v.push(point.clone()))
                .or_insert(vec![point.clone()]);
        }

        let intermediate = mapped.iter().map(|(key, val)| (key, val)).collect::<Vec<_>>();
        let mut result = vec![];
        for (id, point_refs) in intermediate {
            let mut points = point_refs.clone();
            points.sort_by(|a, b| a.device_position.cmp(&b.device_position));
            let mut val_collection: Vec<i32> = vec![];
            for point in points {
                let val = match override_active || point.active {
                    true => point.val.clone(),
                    false => 0,
                };
                val_collection.push(val);
            }
            result.push((id.clone(), val_collection.clone()));
        }
        result
    }

    pub fn controllers(&mut self) -> Result<Vec<NewDevices>, LinuxI2CError> {
        let result = self.get_controller_identities()
            ?.iter()
            .map(|(adr, endpoint_count)| NewDevices {
                adr: adr.clone() as i32,
                endpoint_count: endpoint_count.clone() as i32,
            })
            .collect();
        Ok(result)
    }

    pub fn get_light_levels(&mut self, address: u16) -> Result<Vec<i32>, LinuxI2CError> {
        let light_bits = self.get_light_controller_light_levels(address)?;
        let mut result: Vec<i32> = vec![];
        // should not ever be a non unsigned int number
        let result_lenght = light_bits.len() as u8 / I2C_BYTES_PER_LIGHT;
        for i in 0..result_lenght {
            let offset: usize = (i as usize) * 2;
            result.push((((light_bits[offset] as u16) << 8) | light_bits[offset + 1] as u16) as i32);
        }
        Ok(result)
    }

    pub fn set_light_levels(&mut self, address: u16, values: Vec<i32>) -> Result<(), LinuxI2CError> {
        let endpoint_count = (self.get_controller_endpoint_count(address)?) as usize;
        if endpoint_count < values.len() {
            return Err(
                LinuxI2CError::Io(io::Error::from(ErrorKind::InvalidInput))
            );
        }
        if endpoint_count > values.len() {
            return Err(
                LinuxI2CError::Io(io::Error::from(ErrorKind::InvalidInput))
            );
        }
        if values.iter().any(|val| val.clone() < 0 || val.clone() > LIGHT_LEVEL_MAX) {
            return Err(
                LinuxI2CError::Io(io::Error::from(ErrorKind::InvalidInput))
            );
        }

        let converted_values = values.iter().map(|v| (v.clone() as u16).to_be_bytes()).flatten().collect::<Vec<_>>();
        self.driver.write_bytes(address, converted_values)?;

        Ok(())
    }

    fn get_light_controller_light_levels(&mut self, address: u16) -> Result<Vec<u8>, LinuxI2CError> {
        let endpoint_count = self.get_controller_endpoint_count(address)? * I2C_BYTES_PER_LIGHT;
        let mut result: Vec<u8> = vec![];
        for i in 0..endpoint_count {
            let level = self.driver.read_byte(address, i + I2C_LIGHT_LEVEL_START_OFFSET)?;
            result.push(level);
        }
        Ok(result)
    }

    fn get_controller_identities(&mut self) -> Result<Vec<(u16, u8)>, LinuxI2CError> {
        let mut device_map: Vec<(u16, u8)> = vec![];
        for i in I2C_RANGE_MIN..I2C_RANGE_MAX {
            let device_type = match self.driver.read_byte(i, 0x00) {
                Ok(v) => v,
                Err(_) => continue
            };
            if device_type != 0x10 {
                continue;
            }
            let controlled_count = self.get_controller_endpoint_count(i)?;
            device_map.push((i, controlled_count));
        }
        Ok(device_map)
    }

    fn get_controller_endpoint_count(&mut self, address: u16) -> Result<u8, LinuxI2CError> {
        let identifier = self.driver.read_byte(address, 0x00)?;
        if identifier != 0x10 {
            return Err(
                LinuxI2CError::Io(io::Error::from(ErrorKind::AddrNotAvailable))
            );
        }
        let result = self.driver.read_byte(address, 0x01)?;
        return Ok(result);
    }
}

struct Driver {
    con: LinuxI2CBus,
}

impl Driver {
    pub fn new(connection: u8) -> Result<Self, LinuxI2CError> {
        let con = match LinuxI2CBus::new(format!("/dev/i2c-{}", connection)) {
            Ok(v) => Ok(v),
            Err(_) => {
                match LinuxI2CBus::new(format!("/dev/i2c/{}", connection)) {
                    Ok(v) => Ok(v),
                    Err(err) => Err(err)
                }
            }
        }?;
        Ok(Self { con })
    }

    pub fn read_byte(&mut self, address: u16, value: u8) -> Result<u8, LinuxI2CError> {
        let mut read_data = [0];
        let mut msgs = [
            LinuxI2CMessage::write(&[value]).with_address(address),
            LinuxI2CMessage::read(&mut read_data).with_address(address),
        ];
        self.con.transfer(&mut msgs)?;
        Ok(read_data[0])
    }

    pub fn write_bytes(&mut self, address: u16, values: Vec<u8>) -> Result<(), LinuxI2CError> {
        let mut msgs = [
            LinuxI2CMessage::write(&values).with_address(address),
        ];
        self.con.transfer(&mut msgs)?;
        Ok(())
    }
}
