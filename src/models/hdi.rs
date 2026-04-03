use hidapi::HidApi;
use tracing::info;

pub struct Hdi {
    pub paths: Vec<std::ffi::CString>,
}

impl Hdi {
    pub fn new(vendor_id: u16, product_id: u16) -> Result<Self, anyhow::Error> {
        let api = HidApi::new()?;
        let mut paths = Vec::new();

        for device in api.device_list() {
            if device.vendor_id() == vendor_id && device.product_id() == product_id {
                info!("🔍 Interfaz detectada: #{} | Path: {:?} | UsagePage: 0x{:x} | Usage: 0x{:x}", 
                    device.interface_number(), device.path(), device.usage_page(), device.usage());
                paths.push(device.path().to_owned());
            }
        }

        if paths.is_empty() {
            return Err(anyhow::anyhow!("No se encontró ninguna interfaz para el dispositivo especificado"));
        }

        // Eliminamos duplicados para no abrir la misma interfaz varias veces
        paths.sort();
        paths.dedup();

        Ok(Hdi { paths })
    }
}
