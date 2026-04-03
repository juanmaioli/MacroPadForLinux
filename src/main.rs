mod models;

use hidapi::HidApi;
use std::process::Command;
use tracing::{error, info};
use tracing_subscriber;
use std::sync::{mpsc, Arc};
use std::thread;

use models::{
    Config,
    DeviceEvent,
    Hdi,
};


fn try_exec(cmd: &str) {
    // Ejecuta el comando usando la shell para permitir pipelines, redirecciones, etc.
    match Command::new("sh").arg("-c").arg(cmd).spawn() {
        Ok(_) => {
            // No imprimimos nada aquí para evitar duplicados en el log
        }
        Err(e) => error!("Error al ejecutar '{}': {}", cmd, e),
    }
}

fn main() {
    // Inicializar tracing
    tracing_subscriber::fmt::init();

    // Cargamos la configuración YAML
    let cfg = Arc::new(Config::load_config());

    // Obtenemos los IDs usando el método get_device_ids()
    let (v_id, p_id) = cfg.get_device_ids();

    let hdi = Arc::new(Hdi::new(v_id, p_id).expect("Error al inicializar Hdi"));
    
    // Aplicamos la configuración de los LEDS si existe
    if let Some(ref l) = cfg.led {
        if let Err(e) = hdi.set_led(l.mode, l.r, l.g, l.b) {
            error!("Error al configurar LED: {}", e);
        }
    }

    let (tx, rx) = mpsc::channel();

    // 2. Hilos de Lectura Dinámicos
    let tx_main = tx.clone();
    let hdi_main = hdi.clone();
    
    // Lanzamos un hilo para cada interfaz encontrada
    for path in hdi_main.paths.clone() {
        let tx_interface = tx_main.clone();
        thread::spawn(move || {
            let api = HidApi::new().unwrap();
            match api.open_path(&path) {
                Ok(dev) => {
                    let mut buf = [0u8; 64];
                    loop {
                        if let Ok(res) = dev.read(&mut buf) {
                            if res > 0 {
                                // Solo procesamos si hay algún byte útil distinto de 0
                                if buf.iter().take(res).any(|&b| b != 0) {
                                    if buf[0] == 1 && res >= 4 && buf[3] != 0 {
                                        let _ = tx_interface.send(DeviceEvent::Key(buf[3]));
                                    } else if buf[0] != 1 && res >= 2 {
                                        let code = if buf[1] != 0 { buf[1] } else { buf[0] };
                                        let _ = tx_interface.send(DeviceEvent::Key(code));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => error!("❌ Error al abrir interfaz {:?}: {}", path, e),
            }
        });
    }

    info!("✅ Dispositivo vinculado correctamente.");
    
    // Mostramos la configuración física si está presente
    if let (Some(r), Some(c), Some(w)) = (cfg.rows, cfg.cols, cfg.wheels) {
        info!("🎮 MacroPad: {} ({}x{} teclas, {} ruedas)", 
            cfg.device_id.as_ref().unwrap_or(&"514c:8850".to_string()),
            r, c, w);
    }

    info!("🚀 Escuchando eventos...");

    // 4. Bucle principal de ejecución
    for event in rx {
        match event {
            DeviceEvent::Key(code) => {
                if let Some(cmd) = cfg.keys.as_ref().and_then(|m| m.get(&code)).cloned() {
                    info!("🖱️ Código {}: Ejecutando '{}'", code, cmd);
                    try_exec(&cmd);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::models::Config;
    use std::fs;
    use std::env;
    use std::sync::{Mutex, OnceLock};

    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_lock<'a>() -> &'a Mutex<()> {
        TEST_LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn test_config_loads_empty_when_no_file() {
        let _guard = test_lock().lock().unwrap();
        let orig_dir = env::current_dir().unwrap();
        let orig_home = env::var_os("HOME");
        let tmp = tempfile::tempdir().expect("tempdir");
        env::set_current_dir(tmp.path()).unwrap();
        unsafe { env::remove_var("HOME"); }
        
        let cfg = Config::load_config();
        assert!(cfg.keys.is_none());
        
        env::set_current_dir(orig_dir).unwrap();
        if let Some(home) = orig_home {
            unsafe { env::set_var("HOME", home); }
        }
    }
}
