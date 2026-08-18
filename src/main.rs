use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Error as AnyhowError, Result};
use hidapi::HidApi;

// Currently not configurable from CLI as there is no need.
static UPDATE_PERIOD_MS: u64 = 250;

fn get_thermal_zone(zone_type: &str) -> Result<PathBuf> {
    let basepath = PathBuf::from("/sys/class/thermal");

    for entry in fs::read_dir(&basepath)? {
        if let Ok(entry) = entry {
            let type_path = entry.path().join("type");

            match fs::read_to_string(type_path) {
                Ok(type_str) => {
                    if type_str.trim() == zone_type {
                        return Ok(entry.path().join("temp"));
                    }
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
    }

    Err(AnyhowError::msg("Not found"))
}

fn get_hwmon_sensor(hwmon_name: &str, sensor_label: &str) -> Result<PathBuf> {
    let basepath = PathBuf::from("/sys/class/hwmon");

    for entry in fs::read_dir(&basepath)? {
        let entry = entry?;

        let name_path = entry.path().join("name");

        let name = match fs::read_to_string(name_path) {
            Ok(name) => name.trim().to_string(),
            Err(_) => continue,
        };

        if name != hwmon_name {
            continue;
        }

        for sensor in fs::read_dir(entry.path())? {
            let sensor = sensor?;
            let filename = sensor.file_name();
            let filename = filename.to_string_lossy();

            if !filename.starts_with("temp") || !filename.ends_with("_label") {
                continue;
            }

            let label = fs::read_to_string(sensor.path())?;

            if label.trim() != sensor_label {
                continue;
            }

            let input_filename = filename.replace("_label", "_input");
            let input_path = entry.path().join(input_filename);

            if input_path.exists() {
                return Ok(input_path);
            }
        }
    }

    Err(AnyhowError::msg(format!(
        "Could not find hwmon sensor {}:{}",
        hwmon_name, sensor_label
    )))
}

fn main() -> Result<()> {
    // Parse cmd args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: jonsbo_th <vid:pid> [thermal_zone_type|hwmon:name:label]"
        );
        std::process::exit(1);
    }

    let target_id = &args[1];

    let sensor = args
        .get(2)
        .map(|s| s.as_str().trim())
        .unwrap_or("x86_pkg_temp");

    // Get temperature source.
    //
    // Normal usage:
    //   x86_pkg_temp
    //
    // hwmon usage:
    //   hwmon:k10temp:Tctl
    let temp_path = if let Some(hwmon) = sensor.strip_prefix("hwmon:") {
        let mut parts = hwmon.splitn(2, ':');

        let hwmon_name = parts
            .next()
            .ok_or_else(|| AnyhowError::msg("Missing hwmon name"))?;

        let sensor_label = parts
            .next()
            .ok_or_else(|| AnyhowError::msg("Missing hwmon sensor label"))?;

        get_hwmon_sensor(hwmon_name, sensor_label)?
    } else {
        get_thermal_zone(sensor)
            .context("Could not find a thermal zone with the given type")?
    };

    println!("Using temperature source: {}", temp_path.display());

    // Parse "vid:pid" hex string.
    let ids: Vec<u16> = target_id
        .split(':')
        .map(|s| u16::from_str_radix(s, 16))
        .collect::<Result<Vec<_>, _>>()
        .context("Invalid VID:PID format. Expected hex like 5131:2007")?;

    if ids.len() != 2 {
        return Err(AnyhowError::msg(
            "Invalid VID:PID format. Expected hex like 5131:2007",
        ));
    }

    let (vid, pid) = (ids[0], ids[1]);

    let api = HidApi::new().context("Failed to initialize HID API")?;

    let device = api
        .open(vid, pid)
        .context("Could not open device. Check permissions/udev.")?;

    loop {
        // Allow the occasional failure
        if let Ok(raw_temp) = fs::read_to_string(&temp_path) {
            if let Ok(temp) = raw_temp.trim().parse::<f64>() {
                let mut buf = [0u8; 64];
                buf[0] = 0x01;
                buf[1] = 0x02;
                buf[3] = (temp / 1000.0).max(0.0).min(99.0) as u8;

                device.write(&buf)?;

                device
                    .write(&buf)
                    .context("Failed to write to temperature display")?;

                std::thread::sleep(
                    std::time::Duration::from_millis(UPDATE_PERIOD_MS)
                );
            }
        }
    }
}
