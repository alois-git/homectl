mod lib;
mod registers;

use crate::integrations::huawei_solar::lib::{
    NumericRegister, NumericRegisterTrait, StringRegister,
};
use crate::integrations::huawei_solar::registers::{
    Register, METER_ACTIVE_POWER_L1, METER_ACTIVE_POWER_L2, METER_ACTIVE_POWER_L3, METER_STATUS,
};
use crate::integrations::mqtt::{Mqtt, MqttConfig};
use crate::types::device::{Device, DeviceData, DeviceId, SensorDevice};
use crate::types::event::{Event, TxEventChannel};
use crate::types::integration::{Integration, IntegrationActionPayload, IntegrationId};
use crate::utils::cli::Cli;
use async_trait::async_trait;
use config::Value;
use eyre::{Context, Report};
use modbus::tcp;
use rumqttc::AsyncClient;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use std::{str, thread, time};
use tokio::task;
use tokio::time::sleep;

pub const DEVICE_STATUS_DEFINITIONS: [(u16, &'static str); 30] = [
    (0x0000, "Standby, initializing"),
    (0x0001, "Standby, detecting insulation resistance"),
    (0x0002, "Standby, detecting irradiation"),
    (0x0003, "Standby, grid detecting"),
    (0x0100, "Starting"),
    (0x0200, "On-grid"),
    (0x0201, "Grid Connection, power limited"),
    (0x0202, "Grid Connection, self-derating"),
    (0x0300, "Shutdown, fault"),
    (0x0301, "Shutdown, command"),
    (0x0302, "Shutdown, OVGR"),
    (0x0303, "Shutdown, communication disconnected"),
    (0x0304, "Shutdown, power limited"),
    (0x0305, "Shutdown, manual startup required"),
    (0x0306, "Shutdown, DC switches disconnected"),
    (0x0307, "Shutdown, rapid cutoff"),
    (0x0308, "Shutdown, input underpowered"),
    (0x0401, "Grid scheduling, cosphi-P curve"),
    (0x0402, "Grid scheduling, Q-U curve"),
    (0x0403, "Grid scheduling, PF-U curve"),
    (0x0404, "Grid scheduling, dry contact"),
    (0x0405, "Grid scheduling, Q-P curve"),
    (0x0500, "Spot-check ready"),
    (0x0501, "Spot-checking"),
    (0x0600, "Inspecting"),
    (0x0700, "AFCI self check"),
    (0x0800, "I-V scanning"),
    (0x0900, "DC input detection"),
    (0x0A00, "Running, off-grid charging"),
    (0xA000, "Standby, no irradiation"),
];

pub const METER_REGISTERS: &[Register] = &[
    Register::I16("METER_STATUS", &METER_STATUS),
    Register::I32("METER_ACTIVE_POWER_L1", &METER_ACTIVE_POWER_L1),
    Register::I32("METER_ACTIVE_POWER_L2", &METER_ACTIVE_POWER_L2),
    Register::I32("METER_ACTIVE_POWER_L2", &METER_ACTIVE_POWER_L2),
];

#[derive(Default, Debug, Deserialize, Clone)]
pub struct ModbusConfig {
    host: String,
    port: u16,
    slave_id: u8,
}

pub struct HuaweiSolar {
    client: Option<tcp::Transport>,
    id: IntegrationId,
    event_tx: TxEventChannel,
    config: ModbusConfig,
    cli: Cli,
}

#[async_trait]
impl Integration for HuaweiSolar {
    fn new(
        id: &IntegrationId,
        config: &Value,
        cli: &Cli,
        event_tx: TxEventChannel,
    ) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let config: ModbusConfig = config
            .clone()
            .try_deserialize()
            .wrap_err("Failed to deserialize config of Mqtt integration")?;

        Ok(HuaweiSolar {
            id: id.clone(),
            config,
            cli: cli.clone(),
            event_tx,
            client: None,
        })
    }

    async fn start(&mut self) -> eyre::Result<()> {
        let mut cfg = tcp::Config::default();

        info!(
            "Starting HuaweiSolar server connecting to {}:{}",
            self.config.host, self.config.port
        );
        cfg.modbus_uid = self.config.slave_id;
        cfg.tcp_port = self.config.port;
        cfg.tcp_read_timeout = Some(Duration::from_millis(5000));
        let mut client = match tcp::Transport::new_with_cfg(self.config.host.as_str(), cfg) {
            Ok(client) => client,
            Err(e) => {
                return Err(Report::from(e));
            }
        };

        let event_tx = self.event_tx.clone();
        let integration_id_clone = self.id.clone();

        task::spawn(async move {
            loop {
                let mut register_results: HashMap<String, f64> = HashMap::new();

                for reg in METER_REGISTERS {
                    match reg {
                        Register::I16(name, r) => match r.read(&mut client) {
                            Ok(v) => {
                                register_results.insert(name.to_string(), v[0]);
                            }
                            Err(e) => error!("Error reading {}: {}", r.addr, e),
                        },
                        Register::I32(name, r) => match r.read(&mut client) {
                            Ok(v) => {
                                register_results.insert(name.to_string(), v[0]);
                            }
                            Err(e) => error!("Error reading {}: {}", r.addr, e),
                        },
                    }
                }

                let sensor = DeviceData::Sensor {
                    0: SensorDevice::NumberMap { values: register_results },
                };

                let device = Device {
                    id: DeviceId::new("Huawei"),
                    name: "Huawei".to_string(),
                    integration_id: integration_id_clone.clone(),
                    data: sensor,
                    raw: None,
                };
                let event = Event::ExternalStateUpdate { device };
                event_tx.send(event);
                sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(())
    }

    async fn set_integration_device_state(&mut self, _device: &Device) -> eyre::Result<()> {
        todo!()
    }

    async fn run_integration_action(
        &mut self,
        _payload: &IntegrationActionPayload,
    ) -> eyre::Result<()> {
        todo!()
    }
}
