target/release/low-battery-notification: src/*.rs
	cargo build --release

install: target/release/low-battery-notification
	cp target/release/low-battery-notification /usr/local/bin

bininstall: target/release/low-battery-notification
	cp target/release/low-battery-notification /usr/bin

uninstall:
	rm -f /usr/local/bin/low-battery-notification

.PHONY: install bininstall uninstall
