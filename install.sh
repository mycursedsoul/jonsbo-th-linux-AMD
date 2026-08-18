#!/usr/bin/env sh

set -e

_INSTALL_PATH=/usr/local/bin
_SERVICE_INSTALL_PATH=/etc/systemd/system
_BIN_NAME=jonsbo_th
_SERVICE_NAME=jonsbo.service
_VID_PID=5131:2007
_TEMP_SENSOR=hwmon:k10temp:Tctl
_UPDATE_PERIOD_MS=250

install() {
    echo "Building $_BIN_NAME..."
    cargo build --release

    echo "Installing $_BIN_NAME..."
    sudo install -m 755 "target/release/$_BIN_NAME" \
        "$_INSTALL_PATH/$_BIN_NAME"

    echo "Installing systemd service..."

    sudo tee "$_SERVICE_INSTALL_PATH/$_SERVICE_NAME" > /dev/null <<EOF
[Unit]
Description=Update Jonsbo AIO Display every ${_UPDATE_PERIOD_MS}ms

[Service]
Type=simple
ExecStart=$_INSTALL_PATH/$_BIN_NAME $_VID_PID $_TEMP_SENSOR

[Install]
WantedBy=default.target
EOF

    sudo systemctl daemon-reload
    sudo systemctl enable --now "$_SERVICE_NAME"

    echo "Done. Your water block should now be displaying your CPU temperature."
}

uninstall() {
    echo "Uninstalling $_BIN_NAME..."

    sudo systemctl stop "$_SERVICE_NAME" 2>/dev/null || true
    sudo systemctl disable "$_SERVICE_NAME" 2>/dev/null || true

    sudo rm -f "$_INSTALL_PATH/$_BIN_NAME"
    sudo rm -f "$_SERVICE_INSTALL_PATH/$_SERVICE_NAME"

    sudo systemctl daemon-reload

    echo "Done. Your water block display should go dark in a few seconds."
}

if [ "$1" = "--uninstall" ]; then
    uninstall
else
    install "$1"
fi
