
# ControlNix

ControlNix is a simple GTK4 application built in Rust for controlling essential hardware features such as volume, brightness, WiFi, and Bluetooth on Linux systems. It is designed specifically for use in tiling window managers.

## Features

- **Volume Control**: Adjust system volume.
- **Brightness Control**: Manage screen brightness.
- **WiFi Control**: Connect to and manage WiFi networks.
- **Bluetooth Control**: Connect to and manage Bluetooth devices.

## Dependencies

ControlNix relies on the following utilities:

- **NetworkManager** and **iw** - for managing WiFi networks
- **bluetoothctl** and **bluez** - for managing Bluetooth devices
- **pulseaudio** and **pamixer** - for audio control
- **xbrightness** - for brightness control

## Installation

To install and use ControlNix, ensure the dependencies listed above are installed on your system.

# Install the dependencies 
On Debian/Ubuntu-based systems:
```bash
sudo apt install network-manager iw bluetooth bluez pulseaudio pamixer xbacklight
```

On Arch based systems:
```bash
sudo pacman -S networkmanager iw bluetoothctl bluez pulseaudio pamixer xorg-xbacklight
```

On Fedora:
```bash
sudo dnf install NetworkManager iw bluez pulseaudio pamixer xorg-xbacklight
```

# Clone the repository
git clone https://github.com/himalpoudel334/ControlNix.git
cd ControlNix

# Build the application
cargo build --release

# Run ControlNix
./target/release/ControlNix

# You can copy the compiled binary to your path and add a keybinding to open it with your window manager.
# For example
cp ./target/release/ControlNix ~/.local/bin/

