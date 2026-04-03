use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, env, path::PathBuf};
use tracing::{info, warn, error};


#[derive(Debug, Deserialize)]
pub struct Config {
    pub device_id: Option<String>,
    pub rows: Option<u8>,
    pub cols: Option<u8>,
    pub wheels: Option<u8>,
    pub keys: Option<HashMap<u8, String>>,
}

fn load_config<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Config> {
    info!("📝 Cargando configuración desde {:?}", path.as_ref());
    let s = fs::read_to_string(path)?;
    let cfg: Config = serde_yaml::from_str(&s)?;
    Ok(cfg)
}

impl Config {
    pub fn get_device_ids(&self) -> (u16, u16) {
        if let Some(ref s) = self.device_id {
            if let Some((v, p)) = s.split_once(':') {
                let v_id = u16::from_str_radix(v.trim(), 16).unwrap_or(0x514c);
                let p_id = u16::from_str_radix(p.trim(), 16).unwrap_or(0x8850);
                return (v_id, p_id);
            }
        }
        (0x514c, 0x8850)
    }

    pub fn load_config() -> Config {
        match find_config() {
            Some(path) => match load_config(&path) {
                Ok(c) => c,
                Err(e) => {
                    error!("Se encontró '{}' pero no se pudo cargar: {}. Continuando sin acciones.", path, e);
                    Config {
                        device_id: None,
                        rows: None,
                        cols: None,
                        wheels: None,
                        keys: None,
                    }
                }
            },
            None => {
                warn!("No se encontró config.yaml en el directorio actual ni en ~/.config/kboard/. Continuando sin acciones configuradas.");
                Config {
                    device_id: None,
                    rows: None,
                    cols: None,
                    wheels: None,
                    keys: None,
                }
            }
        }
    }
}

fn find_config() -> Option<String> {
    // Prefer `config.yaml` in current directory
    let cwd = std::path::Path::new("config.yaml");
    if cwd.exists() {
        return Some("config.yaml".to_string());
    }

    // Fallback to $HOME/.config/kboard/config.yaml
    if let Some(home) = env::var_os("HOME") {
        let mut p = PathBuf::from(home);
        p.push(".config/kboard/config.yaml");
        if p.exists() {
            return Some(p.to_string_lossy().into_owned());
        }
    }

    None
}