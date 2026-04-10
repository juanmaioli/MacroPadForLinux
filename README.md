# ⌨️ MacroPad - Controlador de Dispositivos HID

<link rel='icon' href='data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🎹</text></svg>'>

Herramienta ligera escrita en **Rust** para interceptar eventos de dispositivos HID (MacroPads, teclados mecánicos pequeños) y ejecutar comandos personalizados en Linux de forma ultra-rápida.

---

## 🚀 1. Características
*   🛠️ **Configuración Dinámica:** Soporta cualquier dispositivo HID mediante su ID de `lsusb` (formato `VID:PID`).
*   🎡 **Soporte Unificado:** Maneja botones y perillas (knobs) bajo un mismo sistema de códigos.
*   ⚙️ **YAML Intuitivo:** Mapeo sencillo de códigos numéricos a comandos de shell.
*   🐧 **Optimizado para Linux:** Integración fluida con `xdotool`, `pactl` y `playerctl`.
## ⚙️ 2. Instalación Automática
Para compilar e instalar el binario, configurar los permisos USB y activar el servicio en segundo plano (background), simplemente ejecutá:

```bash
chmod +x install.sh
./install.sh
```

### 📋 Comandos Útiles
Una vez instalado, podés gestionar el MacroPad con estos comandos:

*   **Ver actividad en tiempo real:** `journalctl --user -u macropad.service -f`
*   **Reiniciar (si editás el config):** `systemctl --user restart macropad.service`
*   **Detener el servicio:** `systemctl --user stop macropad.service`
*   **Iniciar el servicio:** `systemctl --user start macropad.service`

### Dependencias de Sistema:
...
Asegurante de tener las herramientas necesarias para que tus atajos funcionen:
```bash
sudo apt update && sudo apt install xdotool playerctl libnotify-bin
```

## 🛠️ 3. Configuración (`~/.config/macropad/config.yaml`)
Configuración simplificada usando el ID del dispositivo extraído de `lsusb`:

```yaml
# ⚙️ Configuración del Dispositivo
device_id: "514c:8850"
rows: 4
cols: 3
wheels: 2

# 💡 Configuración de Luces (Modo Dual)
led:
  mode: 1
  r: 255
  g: 0
  b: 0

# ⌨️ Mapeo de Teclas y Ruedas
keys:
  # --- Fila 1 (Superior) ---
  7: "xdotool key ctrl+Home"           # Ir al Inicio
  11: "xdotool key ctrl+End"           # Ir al Final
  15: "xdotool key ctrl+a"             # Seleccionar Todo

# --- Fila 2 Media Superior ---
  6: "xdotool key ctrl+c"             # Copiar
  10: "xdotool key ctrl+v"            # Pegar
  14: "xdotool key ctrl+t"            # Nueva Pesteña

# --- Fila 3 Media Inferior ---
  5: "xdotool key ctrl+z"             # Deshacer
  9: "xdotool key ctrl+y"             # Rehacer
  13: "xdotool key ctrl+w"            # Cerrar Pestaña

  # --- Fila 4 Inferior ---
  4: "xdotool key ctrl+shift+c"       # Copiar
  8: "xdotool key ctrl+shift+v"       # Pegar
  12: "xdotool key ctrl+q"            # Sakir

  # ---  Rueda 1 (Knob Izquierdo) ---
  19: "pactl set-sink-volume @DEFAULT_SINK@ -1%"  # Giro Izquierda: Bajar Vol
  20: "pactl set-sink-mute @DEFAULT_SINK@ toggle" # Presionar: Mute
  21: "pactl set-sink-volume @DEFAULT_SINK@ +1%"  # Giro Derecha: Subir Vol

  # --- Rueda 2 (Knob Derecho) ---
  22: "xdotool key ctrl+minus"    # Giro Izquierda: Zoom Minus
  23: "xdotool key ctrl+0"        # Presionar: Reestablece Zoom
  24: "xdotool key ctrl+plus"     # Giro Derecha: Zoom Plus
```

## 📋 4. Mapa de Teclas (Modelo 3x4 + 2 Knobs)
Basado en tu `config.yaml`, los códigos y acciones actuales son:

| Ubicación | Códigos | Acciones Configuradas |
| :--- | :--- | :--- |
| **Fila 1 (Superior)** | 7, 11, 15 | Ir Inicio, Ir Fin, Seleccionar Todo |
| **Fila 2 (Media-Sup)** | 6, 10, 14 | Copiar, Pegar, Nueva Pestaña |
| **Fila 3 (Media-Inf)** | 5, 9, 13 | Deshacer, Rehacer, Cerrar Pestaña |
| **Fila 4 (Inferior)** | 4, 8, 12 | Copiar/Pegar Terminal, Salir |
| **Rueda 1 (Izquierda)**| 19, 20, 21 | Bajar Vol, Silenciar (Click), Subir Vol |
| **Rueda 2 (Derecha)**  | 22, 23, 24 | Zoom Out, Reset Zoom (Click), Zoom In |

---

## 📖 5. Referencia de Teclas y Comandos
Utilizá estos nombres en tus comandos de `xdotool` dentro del `config.yaml`.

### ⌨️ Modificadores
*   `ctrl`, `shift`, `alt` / `opt`, `win` / `cmd`
*   `rctrl`, `rshift`, `ralt` / `ropt`, `rwin` / `rcmd`

### 🔠 Teclas Estándar
*   `a` - `z`, `1` - `0`
*   `enter`, `escape`, `backspace`, `tab`, `space`
*   `minus`, `equal`, `leftbracket`, `rightbracket`, `backslash`
*   `semicolon`, `quote`, `grave`, `comma`, `dot`, `slash`
*   `capslock`, `printscreen`, `insert`, `delete`
*   `home`, `end`, `pageup`, `pagedown`
*   `right`, `left`, `down`, `up`

---

## 🛡️ 6. Permisos USB (udev)
Para ejecutar sin `sudo` y evitar que las teclas escriban letras en el sistema, creá esta regla en `/etc/udev/rules.d/99-macropad.rules`:

```text
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="514c", ATTRS{idProduct}=="8850", MODE="0666"
SUBSYSTEMS=="usb", ATTRS{idVendor}=="514c", ATTRS{idProduct}=="8850", ENV{ID_INPUT}="", ENV{ID_INPUT_KEYBOARD}="", ENV{LIBINPUT_IGNORE_DEVICE}="1"
```

---
## 🚧 7. Próximos Pasos (TODO)
- [ ] **Soporte de Layers:** Investigar el mapeo de la segunda capa de teclas (Capa 2) activada por el botón físico 'Layer'.
- [ ] **LED Backlight:** Perfeccionar el protocolo de comunicación para el control total de la iluminación RGB (experimental actualmente).

---
*Desarrollado por **Juan Gabriel Maioli*** 🇦🇷  
*Basado en el proyecto original de **Lorenzo Carbonell** ([atareao/kboard](https://github.com/atareao/kboard))*
