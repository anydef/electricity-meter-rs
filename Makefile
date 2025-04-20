TARGET_MACHINE = homelab@192.168.3.50
TARGET_PATH = /home/homelab/electricity-meter-rs/
build:
	cross build --target arm-unknown-linux-musleabi --release

deploy: systemd-stop
	scp ./target/arm-unknown-linux-musleabi/release/electricity_meter_rs $(TARGET_MACHINE):$(TARGET_PATH)
	scp ./target/arm-unknown-linux-musleabi/release/read-serial $(TARGET_MACHINE):$(TARGET_PATH)

run-remote:
	ssh -t $(TARGET_MACHINE) "cd $(TARGET_PATH) && ./read-serial"

run-all: build deploy run-remote

deploy-webserver: build deploy
	ssh -t $(TARGET_MACHINE) "sudo systemctl restart electricity-meter.service"

systemd-stop:
	ssh -t $(TARGET_MACHINE) "sudo systemctl stop electricity-meter.service"

systemd-start:
	ssh -t $(TARGET_MACHINE) "sudo systemctl start electricity-meter.service"

update-systemd: systemd-stop
	scp ./root/etc/systemd/system/electricity-meter.service $(TARGET_MACHINE):$(TARGET_PATH)
	ssh -t $(TARGET_MACHINE) "sudo cp $(TARGET_PATH)/electricity-meter.service /etc/systemd/system/"
	ssh -t $(TARGET_MACHINE) "sudo systemctl start electricity-meter.service"
