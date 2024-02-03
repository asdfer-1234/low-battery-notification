use battery;
use libnotify;
use std::{env, fs, thread, time::Duration};

struct Config {
	urgency: libnotify::Urgency,
	update_time: f64,
	alert_battery: f64,
	message: String,
}

fn get_battery(manager: &battery::Manager) -> Result<Option<battery::Battery>, battery::Error> {
	let mut batteries = manager.batteries()?;
	let battery = match batteries.next() {
		Some(Ok(battery)) => Some(battery),
		Some(Err(i)) => return Err(i),
		None => None,
	};

	Ok(battery)
}

fn get_config() -> Config {
	let default = Config {
		urgency: libnotify::Urgency::Critical,
		update_time: 1.0,
		alert_battery: 0.15,
		message: "Low battery".to_string(),
	};
	let config_path = match env::var("XDG_CONFIG_HOME") {
		Ok(i) => i,
		Err(..) => match env::var("HOME") {
			Ok(i) => i + "/.config",
			Err(..) => return default,
		},
	} + "/low_battery_notification.yaml";

	let config_file = match fs::read_to_string(config_path) {
		Ok(i) => i,
		Err(..) => return default,
	};

	let mut config = default;

	let config_yaml = &match yaml_rust::YamlLoader::load_from_str(&config_file) {
		Ok(i) => i,
		Err(..) => {
			eprintln!("Failed to parse config file. Using default configuration.");
			return config;
		}
	}[0];

	if let Some(urgency_str) = config_yaml["urgency"].as_str() {
		config.urgency = match urgency_str {
			"low" => libnotify::Urgency::Low,
			"normal" => libnotify::Urgency::Normal,
			"critical" => libnotify::Urgency::Critical,
			_ => {
				eprintln!("\"urgency\" value not recognizable. Using default configuration for \"urgency\".");
				config.urgency
			}
		}
	}

	if let Some(update_time) = config_yaml["update_time"].as_f64() {
		config.update_time = update_time;
	}

	if let Some(alert_battery) = config_yaml["alert_battery"].as_f64() {
		config.alert_battery = alert_battery;
	}

	if let Some(message) = config_yaml["message"].as_str() {
		config.message = message.to_string();
	}

	config
}

fn main() {
	let manager = match battery::Manager::new() {
		Ok(i) => i,
		Err(..) => {
			eprintln!("Unable to get manager. Stop.");
			return;
		}
	};
	let mut notification_available = true;

	let mut battery = match get_battery(&manager) {
		Err(..) => {
			eprintln!("Unable to get battery. Stop.");
			return;
		}
		Ok(i) => match i {
			Some(j) => j,
			None => {
				eprintln!("No batteries detected. Stop.");
				return;
			}
		},
	};

	let config = get_config();

	if let Err(..) = libnotify::init(&config.message) {
		eprintln!("Failed to initialize notification. Stop.");
		return;
	}
	let low_battery_notification = libnotify::Notification::new("Low battery", None, None);
	low_battery_notification.set_urgency(config.urgency);

	loop {
		if notification_available
			&& battery.state() == battery::State::Discharging
			&& battery.energy().value / battery.energy_full().value
				<= config.alert_battery as f32
		{
			if let Err(..) = low_battery_notification.show() {
				// Users are dumb. Always let them know the information.
				eprintln!(
					"Failed to show notification. {} btw if you aren't aware",
					config.message
				);
			}
			notification_available = false;
		} else if battery.state() != battery::State::Discharging {
			notification_available = true;
			if let Err(..) = low_battery_notification.close() {
				eprintln!("Failed to close notification");
			}
		}
		thread::sleep(Duration::from_secs_f64(config.update_time));
		if let Err(..) = manager.refresh(&mut battery) {
			eprintln!("Unable to refresh batteries");
		}
	}
}
