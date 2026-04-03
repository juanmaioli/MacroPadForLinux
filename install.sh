#!/bin/bash

# 🎹 Script de Instalación MacroPad

echo "🚀 Compilando MacroPad en modo Release..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Error en la compilación. Abortando."
    exit 1
fi

echo "📦 Instalando binario en /usr/local/bin..."
sudo cp ./target/release/macropad /usr/local/bin/macropad
sudo chmod +x /usr/local/bin/macropad

echo "⚙️ Configurando directorio de usuario..."
mkdir -p ~/.config/macropad
if [ ! -f ~/.config/macropad/config.yaml ]; then
    cp ./config.yaml ~/.config/macropad/config.yaml
    echo "✅ Archivo config.yaml creado en ~/.config/macropad/"
else
    echo "ℹ️ El archivo config.yaml ya existe, no se ha sobrescrito."
fi

echo "🛡️ Configurando reglas de udev para el dispositivo 514c:8850..."
echo 'SUBSYSTEM=="hidraw", ATTRS{idVendor}=="514c", ATTRS{idProduct}=="8850", MODE="0666"' | sudo tee /etc/udev/rules.d/99-macropad.rules
sudo udevadm control --reload-rules
sudo udevadm trigger

echo "✨ ¡Instalación completada con éxito!"
echo "🎹 Ya podés ejecutar 'macropad' desde cualquier terminal."
