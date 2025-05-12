use btleplug::{
    api::{
        bleuuid::uuid_from_u16, Central, Characteristic, Manager as _, Peripheral as _, ScanFilter,
        Service,
    },
    platform::{Adapter, Manager, Peripheral},
};
use tracing::{debug, error};
use uuid::Uuid;

const LIGHT_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFE9);
const SERVICE_UUID: Uuid = uuid_from_u16(0xFFE0);

pub struct DeviceManager {
    central: Adapter,
    target_ble_conf: BleConf,
    peripheral: Option<Peripheral>,
    service: Option<Service>,
    target_characteristic: Option<Characteristic>,
    is_connected: bool,
}

impl DeviceManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // 创建蓝牙管理器
        let manager = Manager::new().await?;
        // 获取第一个蓝牙适配器
        let adapters = manager.adapters().await?;
        if adapters.is_empty() {
            error!("No Bluetooth adapter found.");
            return Err("No Bluetooth adapter found.".into());
        }
        let central = adapters
            .into_iter()
            .nth(0)
            .expect("No Bluetooth adapter found");

        // 得到到适配器的属性
        Ok(Self {
            central,
            target_ble_conf: BleConf::default(),
            peripheral: None,
            service: None,
            target_characteristic: None,
            is_connected: false,
        })
    }

    // 扫描并过滤特定设备
    pub async fn scan_and_connect_ble(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Scanning the target Bluetooth device");
        self.central.start_scan(ScanFilter::default()).await?;
        std::thread::sleep(std::time::Duration::from_secs(3));
        debug!("Scan complete.");

        // 查找特定设备（例如，设备名称为 "MyDevice"）
        if let Some(p) = find_target_peri(&self.central, self.target_ble_conf.name()).await {
            // 连接设备
            debug!("Connecting to the target Bluetooth device...");
            p.connect().await?;
            self.is_connected = true;
            self.peripheral = Some(p);
            debug!("Connected to the target Bluetooth device.");
        } else {
            error!("Target device not found.");
            return Err("Target device not found.".into());
        };
        Ok(())
    }

    // 断开连接
    pub async fn disconnect_ble(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_connected() {
            if let Some(p) = &mut self.peripheral {
                p.disconnect().await?;
                debug!("Disconnected from the target Bluetooth device.");
            }
        }
        debug!("Device disconnected.");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub async fn connect_service(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(p) = &mut self.peripheral {
            p.discover_services().await.unwrap();
            if let Some(service) = p.services().into_iter().find(|s| s.uuid == SERVICE_UUID) {
                debug!("Found service: {:?}", &service.uuid);
                self.service = Some(service);
            } else {
                error!("Service not found");
                return Err("Service not found".into());
            }
        }
        Ok(())
    }

    pub async fn get_characteristic(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(service) = &self.service {
            if let Some(target_char) = service
                .characteristics
                .iter()
                .find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
            {
                self.target_characteristic = Some(target_char.clone());
                debug!("Found characteristic: {:?}", target_char.uuid);
            } else {
                error!("Characteristic not found");
                return Err("Characteristic not found".into());
            }
        }
        Ok(())
    }

    pub async fn read(&self) -> u32 {
        if let Some(p) = &self.peripheral {
            if let Some(service) = &self.service {
                if let Some(target_char) = service
                    .characteristics
                    .iter()
                    .find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
                {
                    // 读取特征值
                    let value = p.read(target_char).await.unwrap();
                    debug!("Read value: {:?}", value);
                    return value[0] as u32;
                }
            }
        }
        0
    }
}

async fn find_target_peri(central: &Adapter, target_ble_name: &str) -> Option<Peripheral> {
    debug!("Searching for target Bluetooth device: {}", target_ble_name);
    // 遍历所有外设，查找目标设备
    for p in central.peripherals().await.unwrap() {
        if let Some(ble_name) = p.properties().await.unwrap().unwrap().local_name {
            if ble_name.contains(target_ble_name) {
                debug!("Found target Bluetooth device: {}", ble_name);
                return Some(p);
            } else {
                debug!("Found Bluetooth device: {}", ble_name);
            }
            debug!("Found target Bluetooth device: {}", target_ble_name);
        }
    }
    debug!("Target Bluetooth device not found: {}", target_ble_name);
    None
}

struct BleConf {
    id: String,
    name: String,
    services: Vec<uuid::Uuid>,
}

impl Default for BleConf {
    fn default() -> Self {
        // 默认使用esp32的设备信息
        Self {
            id: String::new(),
            name: "123321".to_string(),
            services: Vec::new(),
        }
    }
}

impl BleConf {
    fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            services: Vec::new(),
        }
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn services(&self) -> &[uuid::Uuid] {
        &self.services
    }
}
