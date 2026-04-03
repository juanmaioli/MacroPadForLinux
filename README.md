# ⌨️ KBoard - Controlador de MacroPad

<link rel='icon' href='data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🎹</text></svg>'>

Herramienta ligera escrita en **Rust** para interceptar eventos de dispositivos HID (MacroPads, teclados mecánicos pequeños) y ejecutar comandos personalizados de forma ultra-rápida.

---

## 🚀 1. Características
*   🛠️ **Configuración Dinámica:** Soporta cualquier dispositivo HID mediante su ID de `lsusb`.
*   🎡 **Soporte Multi-Rueda:** Capacidad para gestionar múltiples perillas (knobs).
*   ⚙️ **YAML Intuitivo:** Mapeo sencillo de códigos de tecla a comandos de shell.
*   🐧 **Optimizado para Linux:** Integración con `xdotool`, `pactl` y `playerctl`.

## ⚙️ 2. Instalación
Asegurante de tener las dependencias necesarias en tu sistema Linux:

```bash
sudo apt update && sudo apt install xdotool playerctl libnotify-bin
```

## 🛠️ 3. Configuración (`config.yaml`)
Ahora podés copiar el ID directamente de la salida de `lsusb`:

```yaml
# ⚙️ Configuración del Dispositivo
device_id: "514c:8850"

# ⌨️ Mapeo de Teclas y Ruedas
keys:
  7: "xdotool key ctrl+c"             # Copiar
  19: "pactl set-sink-volume @DEFAULT_SINK@ +5%" # Rueda Vol+
```

## 🛡️ 4. Permisos USB (udev)
Para ejecutar sin `sudo`, crea una regla en `/etc/udev/rules.d/99-macropad.rules`:

```text
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="514c", ATTRS{idProduct}=="8850", MODE="0666"
```

---
*Desarrollado por **Juan Gabriel Maioli*** 🇦🇷
