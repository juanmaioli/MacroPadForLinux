# ⌨️ MacroPad - Controlador de Dispositivos HID (v0.8.1)

<link rel='icon' href='data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🎹</text></svg>'>

[Documentación Oficial](https://github.com/juanmaioli/MacroPad)

Herramienta ligera y de alto rendimiento escrita en **Rust** (Edición 2024) para interceptar eventos de dispositivos HID (MacroPads, teclados mecánicos pequeños) y ejecutar comandos personalizados en Linux de forma ultra-rápida.

---

## 🚀 1. Características Principales
*   🛠️ **Configuración Dinámica:** Soporta cualquier dispositivo HID mediante su ID de `lsusb` (formato `VID:PID`).
*   🎡 **Soporte Unificado:** Maneja botones y perillas (knobs) bajo un mismo sistema de códigos de eventos.
*   ⚙️ **YAML Intuitivo:** Mapeo sencillo de códigos numéricos a comandos de shell potentes.
*   🐧 **Optimizado para Linux:** Integración nativa con `xdotool`, `pactl` y `playerctl`.
*   🛡️ **Seguridad:** Gestión de permisos mediante reglas `udev` para evitar interferencias con el sistema.

---

## ⚙️ 2. Instalación y Uso
Para compilar e instalar el binario, configurar los permisos USB y activar el servicio en segundo plano (`systemd`), ejecutá:

```bash
chmod +x install.sh
./install.sh
```

### 📋 Gestión del Servicio
Podés controlar el estado del controlador con los siguientes comandos:

| Acción | Comando |
| :--- | :--- |
| **Ver Logs** | `journalctl --user -u macropad.service -f` |
| **Reiniciar** | `systemctl --user restart macropad.service` |
| **Detener** | `systemctl --user stop macropad.service` |
| **Estado** | `systemctl --user status macropad.service` |

### 📦 Dependencias Sugeridas
Para aprovechar al máximo los atajos, asegurate de tener instaladas estas utilidades:
```bash
sudo apt update && sudo apt install xdotool playerctl libnotify-bin
```

---

## 🛠️ 3. Configuración (`~/.config/macropad/config.yaml`)
El archivo de configuración utiliza un formato YAML simple para vincular eventos HID con comandos.

```yaml
# ⚙️ Identificación del Hardware
device_id: "514c:8850"
rows: 4
cols: 3
wheels: 2

# ⌨️ Mapeo de Acciones
keys:
  7: "xdotool key ctrl+c"             # Copiar
  19: "pactl set-sink-volume @DEFAULT_SINK@ +5%" # Subir Volumen (Rueda)
```

---

## 📋 4. Mapa de Teclas Estándar (3x4 + 2 Knobs)
Basado en la configuración predeterminada para el modelo soportado:

| Ubicación | Códigos HID | Acción Sugerida |
| :--- | :--- | :--- |
| **Fila 1 (Superior)** | 7, 11, 15 | Inicio / Fin / Seleccionar Todo |
| **Fila 2 (Media-Sup)** | 6, 10, 14 | Copiar / Pegar / Nueva Pestaña |
| **Fila 3 (Media-Inf)** | 5, 9, 13 | Deshacer / Rehacer / Cerrar Pestaña |
| **Fila 4 (Inferior)** | 4, 8, 12 | Terminal Shortcuts / Salida |
| **Rueda 1 (Izquierda)**| 19, 20, 21 | Control de Volumen y Silencio |
| **Rueda 2 (Derecha)**  | 22, 23, 24 | Zoom y Reset de Visualización |

---

## 🛡️ 5. Reglas de udev (`/etc/udev/rules.d/99-macropad.rules`)
Es crítico configurar estas reglas para que el dispositivo no envíe caracteres extraños al sistema:

```text
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="514c", ATTRS{idProduct}=="8850", MODE="0666"
SUBSYSTEMS=="usb", ATTRS{idVendor}=="514c", ATTRS{idProduct}=="8850", ENV{ID_INPUT}="", ENV{ID_INPUT_KEYBOARD}="", ENV{LIBINPUT_IGNORE_DEVICE}="1"
```

---

## 📈 6. Historial de Versiones
| Versión | Tipo | Descripción |
| :--- | :--- | :--- |
| **v0.8.1** | 🐛 Patch | Sincronización de manifiesto y mejoras visuales en README. |
| **v0.8.0** | ✨ Feat | Soporte para ejecución en segundo plano mediante `systemd`. |
| **v0.7.3** | 🐛 Fix | Corregida la doble escritura mediante reglas de udev avanzadas. |

---

## 🚧 7. Próximos Pasos (ROADMAP)
- [ ] **Soporte de Layers:** Implementar múltiples capas de comandos (Capa 1, Capa 2).
- [ ] **Protocolo LED:** Perfeccionar la secuencia `0xa1, 0xb0, 0xaa` para control RGB total.

---
*Desarrollado por **Juan Gabriel Maioli*** 🇦🇷  
*Inspirado en el trabajo de **Lorenzo Carbonell** ([atareao](https://github.com/atareao))*
