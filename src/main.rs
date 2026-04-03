mod models;

use hidapi::HidApi;
use std::process::Command;
use tracing::{debug, error, info};
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
        Ok(child) => {
            // No esperamos al comando, lo dejamos correr en background
            info!("Ejecutando: '{}' (pid={})", cmd, child.id());
        }
        Err(e) => error!("Error al ejecutar '{}': {}", cmd, e),
    }
}

fn main() {
    // Inicializar tracing (puede usar RUST_LOG para filtrar)
    tracing_subscriber::fmt::init();

    // Cargamos la configuración YAML. Buscamos en el directorio actual primero,
    // luego en `$HOME/.config/kboard/config.yaml`.
    let cfg = Arc::new(Config::load_config());

    // Obtenemos los IDs usando el nuevo método get_device_ids()
    let (v_id, p_id) = cfg.get_device_ids();

    let hdi = Arc::new(Hdi::new(v_id, p_id).expect("Error al inicializar Hdi"));
    let (tx, rx) = mpsc::channel();

    // 2. Hilo para las TECLAS
    // Hilo unificado o procesador de eventos
    let tx_main = tx.clone();
    let hdi_main = hdi.clone();
    thread::spawn(move || {
        let api = HidApi::new().unwrap();
        // Abrimos la interfaz 1 que es la que está mandando datos
        let p_keys = &hdi_main.p_keys;
        let p_wheel = &hdi_main.p_wheel;
        
        // Función interna para procesar los buffers
        let process_buffer = |buf: &[u8], res: usize, transmitter: &mpsc::Sender<DeviceEvent>| {
            if res > 0 && buf[0] == 1 { // Report ID 1: Todo el dispositivo
                if res >= 4 && buf[3] != 0 {
                    let code = buf[3];
                    info!("🖱️ Evento detectado: Código {}", code);
                    let _ = transmitter.send(DeviceEvent::Key(code));
                }
            }
        };

        // Escuchamos en ambas interfaces por si acaso
        let dev_keys = api.open_path(p_keys).expect("No se pudo abrir p_keys");
        let dev_wheel = api.open_path(p_wheel).expect("No se pudo abrir p_wheel");

        let tx_k = tx_main.clone();
        thread::spawn(move || {
            let mut buf = [0u8; 64];
            while let Ok(res) = dev_keys.read(&mut buf) {
                process_buffer(&buf, res, &tx_k);
            }
        });

        let mut buf = [0u8; 64];
        while let Ok(res) = dev_wheel.read(&mut buf) {
            process_buffer(&buf, res, &tx_main);
        }
    });

    info!("✅ Dispositivo vinculado correctamente.");
    info!("🚀 Escuchando eventos... ");

    // 4. Bucle principal de ejecución
    for event in rx {
        match event {
            DeviceEvent::Key(code) => {
                if let Some(cmd) = cfg.keys.as_ref().and_then(|m| m.get(&code)).cloned() {
                    info!("🖱️ Ejecutando acción para código {}", code);
                    try_exec(&cmd);
                } else {
                    debug!("Código sin acción configurada: {}", code);
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
        // Test that Config::load_config() returns empty config when no file exists
        let orig_dir = env::current_dir().unwrap();
        let orig_home = env::var_os("HOME");
        let tmp = tempfile::tempdir().expect("tempdir");
        env::set_current_dir(tmp.path()).unwrap();
        unsafe { env::remove_var("HOME"); } // Ensure no HOME fallback
        
        let cfg = Config::load_config();
        assert!(cfg.keys.is_none());
        assert!(cfg.wheel.is_none());
        
        // Restore original state
        env::set_current_dir(orig_dir).unwrap();
        if let Some(home) = orig_home {
            unsafe { env::set_var("HOME", home); }
        }
    }

    #[test]
    fn test_config_prefers_cwd() {
        let _guard = test_lock().lock().unwrap();
        let orig_dir = env::current_dir().unwrap();
        let tmp = tempfile::tempdir().expect("tempdir");
        env::set_current_dir(tmp.path()).unwrap();
        
        // Create config.yaml in current directory
        let yaml = "keys:\n  3: \"echo test\"\nwheel:\n  1: \"echo wheel\"\n";
        fs::write(tmp.path().join("config.yaml"), yaml).unwrap();
        
        let cfg = Config::load_config();
        assert!(cfg.keys.is_some());
        assert_eq!(cfg.keys.unwrap().get(&3).map(String::as_str), Some("echo test"));
        
        env::set_current_dir(orig_dir).unwrap();
    }
}
