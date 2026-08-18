* Jonsbo TH-240/360 temperature display on Linux AMD BAZZITE
* Picked up this project and modified it in order to work on my Bazzite install on a AM4 B550 motherboard using hwmon:k10temp:Tctl sensor info.
***** I AM NOT A CODER, i got helped by IA.*****
* ORIGINAL https://github.com/htkhiem/jonsbo-th-linux , kudos to htkhiem
---

Minimal, no-UI Linux app to drive the temperature display on [Jonsbo TH-series](https://www.jonsbo.com/en/products/TH-360--.html) AIO water blocks. Basically functionally equivalent to their official Windows app, but for Linux and uses about 1MB of RAM only.

## Disclaimer

**Use this at your own risk.**.
* While this app only makes use of basic USB HID functionality (and therefore should be relatively safe compared to those having to use SMBus/i2c), there might still be a risk of malfunctioning or complete bricking of your AIO's display, stemming from possible differences between units and/or manufacturing batches. I have tested this on my own TH-240, but that does not necessarily translate to _every single_ Jonsbo TH-series unit sold out there.

As with GPLv3, this software is provided without warranty. The software author or license can not be held liable for any damages inflicted by the software.

## Instructions

1. Check whether your Jonsbo AIO is compatible by looking for its USB VID:PID in `lsusb`. This command might not be available out of the box depending on your distro. For example, on Arch Linux you'll need to install `usbutils` first.

    ```sh
    $ lsusb                                            
    Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
    (...) 
    Bus 001 Device 006: ID 5131:2007 MSR MSR-101U Mini HID magnetic card reader
    (...)
    ```
    The TH-series water block displays should show up as some sort of "Mini HID magnetic card reader" (don't ask me, I don't know why either). If you see any such device in your `lsusb` result, especially with ID `5131:2007`, you're good to go. If you are **really sure** that your AIO is one of those two Jonsbo models but no such device exists, go to the FAQ section.

2. As a prerequisite, ensure you have a Rust toolchain installed:

    ``` sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```


3. Install required libraries for rust
    ``` sh
    sudo rpm-ostree install systemd-devel
    ```
    
4. Clone & compile:

    ``` sh
    git clone https://github.com/mycursedsoul/jonsbo-th-linux-AMD.git
    cd jonsbo-th-linux
    cargo build --release
    ```
    
5. Install & register with `systemd` to run it on startup.

    ```sh
    chmod a+x install.sh
    ./install.sh
    ```
*   The display on the water block should now light up & display your CPU temperature, updated in 250ms, it's the slowest number it can do without artifacting. **The display only has two digits & can only indicate up to 99°C.**
*   Please be mindful this ONLY CONTROLS THE DISPLAY, NOT THE FAN OR PUMP PROFILES!!!
    
    The above script installs for all users by default.
    
## Uninstallation

    ```sh
    chmod a+x install.sh
    ./install.sh --uninstall
    ```

## Manual start/stop

The above installation script will set up a `systemd` service named `jonsbo` that runs on boot. If desired, you may control it as with any other service:

    ```sh
    # Stop
    systemctl stop jonsbo
    # Disable autostart
    systemctl disable jonsbo
    # Start
    systemctl start jonsbo
    # Enable autostart & start immediately
    systemctl enable --now jonsbo
    ```


****
