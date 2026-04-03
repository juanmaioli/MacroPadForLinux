# ⌨️ MacroPad - Controlador de Dispositivos HID

<link rel='icon' href='data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🎹</text></svg>'>

Herramienta ligera escrita en **Rust** para interceptar eventos de dispositivos HID y ejecutar comandos personalizados en Linux.

---

## 🚀 1. Características
*   🛠️ **Configuración Dinámica:** Soporta cualquier dispositivo HID mediante su ID de `lsusb` (formato `VID:PID`).
*   🎡 **Soporte Unificado:** Maneja botones y perillas (knobs) bajo un mismo sistema de códigos.
*   ⚙️ **YAML Intuitivo:** Mapeo sencillo de códigos numéricos a comandos de shell.
*   🐧 **Optimizado para Linux:** Integración fluida con `xdotool`, `pactl` y `playerctl`.

## ⚙️ 2. Instalación
Asegurante de tener las dependencias necesarias en tu sistema Linux:

```bash
sudo apt update && sudo apt install xdotool playerctl libnotify-bin
```

## 🛠️ 3. Configuración (`config.yaml`)
Configuración simplificada usando el ID del dispositivo extraído de `lsusb`:

```yaml
# ⚙️ Configuración del Dispositivo
device_id: "514c:8850"
rows: 4
cols: 3
wheels: 2

# ⌨️ Mapeo de Acciones (Botones y Ruedas)
keys:
  7: "xdotool key ctrl+c"             # Copiar
  19: "pactl set-sink-volume @DEFAULT_SINK@ +5%" # Rueda Derecha
```

## 📋 4. Mapa de Teclas (Modelo 3x4 + 2 Knobs)
Basado en el dispositivo `514c:8850`, los códigos detectados son:

| Tipo | Ubicación | Código | Acción Configurada |
| :--- | :--- | :--- | :--- |
| **Tecla** | Fila 1 (Sup) | 4, 5, 6 | Deshacer, Rehacer, Cortar |
| **Tecla** | Fila 2 (Med) | 7, 8, 9 | Copiar, Pegar, Seleccionar Todo |
| **Tecla** | Fila 3 (Inf) | 10, 11, 12 | Guardar, Cambiar Ventana, Abrir Carpeta |
| **Tecla** | Fila 4 (Extra) | 13, 14, 15 | Silenciar, Play/Pausa, Notificación |
| **Rueda 1** | Izquierda | 19, 20, 23 | Vol+, Vol-, Mute (Click) |
| **Rueda 2** | Derecha | 21, 22, 24 | Zoom In, Zoom Out, Reset Zoom (Click) |

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

### 🛠️ Teclas de Función y Especiales
*   `f1` - `f24`
*   `macbrightnessdown`, `macbrightnessup`
*   `numlock`, `numpad1` - `numpad0`, `numpadenter`, `numpaddot`
*   `application`, `power`

### 🎵 Teclas Multimedia
*   `next`, `previous` / `prev`, `stop`, `play`, `mute`
*   `volumeup`, `volumedown`, `favorites`, `calculator`, `screenlock`

### 🖱️ Acciones de Mouse (xdotool)
*   `wheel(-100)`, `click(left+right)`, `move(5,0)`, `drag(left+right,0,5)`

---

## 🛡️ 6. Permisos USB (udev)
Para ejecutar sin `sudo`, crea una regla en `/etc/udev/rules.d/99-macropad.rules`:

```text
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="514c", ATTRS{idProduct}=="8850", MODE="0666"
```

---
*Desarrollado por **Juan Gabriel Maioli*** 🇦🇷  
*Basado en el proyecto original de **Lorenzo Carbonell** ([atareao/kboard](https://github.com/atareao/kboard))*
