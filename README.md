# Low Battery Notification
Low battery notifier written in rust.
For Linux.
## Dependencies
- rust
- libnotify
- A physical battery (probably?)
## Installation
To Install:\
`$ rustup version stable` Might be required before running this command
```sh
# make install
```
To Uninstall:
```sh
# make uninstall
```
The binary file will be stored in `/usr/local/bin`
## Usage
```sh
low-battery-notification
```
## Configuration
This program will look for the system's `XDG_CONFIG_HOME` variable first, and if the variable does not exist, it will then try to find the `HOME` then add a `.config` directory on the way.\
On the configuraiton directory, the program will search for `low_battery_notification.yaml` file and read the configuration there.
### Configuratable things
- `message` : Title of the notification
- `urgency` : Notification urgency used by libnotify
Can be selected between `critical`, `normal`, `low`
- `update_time` : Delay between checking battery status
- `alert_battery` : Battery ratio it will alert on.

### Example configuration
```yaml
message: Low battery
urgency: critical
update_time: 1
alert_battery: 0.15
```
# License
GNU Public License Version 3
