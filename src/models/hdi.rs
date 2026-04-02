
use hidapi::HidApi;
use tracing::info;

pub struct Hdi {
    // Definimos los IDs del dispositivo
    pub p_keys: std::ffi::CString,
    pub p_wheel: std::ffi::CString,
}

// Definimos los IDs del dispositivo
const VENDOR_ID: u16 = 0x514c;
const PRODUCT_ID: u16 = 0x8850;


impl Hdi {
    pub fn new() -> Result<Self, anyhow::Error> {
        let api = HidApi::new()?;
        let mut path_keys = None;
        let mut path_wheel = None;

        for device in api.device_list() {
            if device.vendor_id() == VENDOR_ID && device.product_id() == PRODUCT_ID {
                info!("🔍 Interfaz: #{} | Path: {:?} | UsagePage: 0x{:x} | Usage: 0x{:x}", 
                    device.interface_number(), device.path(), device.usage_page(), device.usage());
                
                match device.interface_number() {
                    0 => path_keys = Some(device.path().to_owned()),
                    1 => path_wheel = Some(device.path().to_owned()),
                    2 => path_wheel = Some(device.path().to_owned()),
                    _ => {}
                }
                
                // Si encontramos una interfaz con UsagePage 0xff00 o similar, suele ser la de control
                if device.usage_page() >= 0xff00 {
                    info!("✨ Detectada interfaz de control especial (ruedas/config)");
                    path_wheel = Some(device.path().to_owned());
                }
            }
        }

        // Si no encontramos la 1 o 2, intentamos usar la 0 para ambos (algunos modelos lo hacen así)
        let p_keys = path_keys.ok_or_else(|| anyhow::anyhow!("No se encontró ninguna interfaz (Int 0)"))?;
        let p_wheel = path_wheel.unwrap_or_else(|| p_keys.clone()); // Fallback a la misma de teclas

        Ok(Hdi { p_keys, p_wheel })
    }
}
