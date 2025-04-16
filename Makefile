TARGET_MACHINE = homelab@192.168.3.50
TARGET_PATH = /home/homelab/electricity-meter-rs/
build:
	cross build --target arm-unknown-linux-musleabi --release

deploy:
	scp ./target/arm-unknown-linux-musleabi/release/electricity-meter-rs $(TARGET_MACHINE):$(TARGET_PATH)
	scp ./target/arm-unknown-linux-musleabi/release/read-serial $(TARGET_MACHINE):$(TARGET_PATH)

run-remote:
	ssh -t $(TARGET_MACHINE) "cd $(TARGET_PATH) && ./read-serial"

run-all: build deploy run-remote
