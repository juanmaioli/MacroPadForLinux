use hidapi::HidApi;
use tracing::{info, warn};

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

    pub fn set_led(&self, mode: u8, _r: u8, _g: u8, _b: u8) -> Result<(), anyhow::Error> {
        let api = HidApi::new()?;
        for device in api.device_list() {
            if device.vendor_id() == 0x514c && device.product_id() == 0x8850 && 
               (device.usage_page() == 0xff00 || device.interface_number() == 0) {
                
                if let Ok(dev) = api.open_path(device.path()) {
                    // Secuencia de 3 pasos descubierta en k8890
                    
                    // 1. Apertura de sesión
                    let mut buf1 = [0u8; 65];
                    let msg1 = [0x03, 0xa1, 0x01, 0, 0, 0, 0, 0, 0];
                    buf1[0] = 0x00;
                    buf1[1..1+msg1.len()].copy_from_slice(&msg1);
                    let _ = dev.write(&buf1);

                    // 2. Configuración de Modo
                    let mut buf2 = [0u8; 65];
                    let msg2 = [0x03, 0xb0, 0x18, mode, 0, 0, 0, 0, 0];
                    buf2[0] = 0x00;
                    buf2[1..1+msg2.len()].copy_from_slice(&msg2);
                    let _ = dev.write(&buf2);

                    // 3. Cierre y Aplicación
                    let mut buf3 = [0u8; 65];
                    let msg3 = [0x03, 0xaa, 0xa1, 0, 0, 0, 0, 0, 0];
                    buf3[0] = 0x00;
                    buf3[1..1+msg3.len()].copy_from_slice(&msg3);
                    
                    match dev.write(&buf3) {
                        Ok(_) => {
                            info!("💡 Secuencia LED k8890 completada. Modo: {:#04x}", mode);
                            return Ok(());
                        },
                        Err(e) => return Err(anyhow::anyhow!("Error al finalizar secuencia LED: {}", e)),
                    }
                }
            }
        }
        warn!("⚠️ No se encontró la interfaz adecuada para los LEDs");
        Ok(())
    }
}
