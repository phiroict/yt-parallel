init_arm:
	rustup target add x86_64-unknown-linux-gnu
	brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
	echo "[target.x86_64-unknown-linux-gnu]" >> $(USER)/.cargo/config.toml
	echo "linker = 'x86_64-unknown-linux-gnu-gcc'" >> $(USER)/.cargo/config.toml
clean:
	cargo clean
init:
	cargo install --force cargo-audit
	cargo install --force cargo-outdated
	cargo install --force cargo-update
	cargo install --force cargo-semver-tool
init_arch:
	pacman -S which yt-dlp --needed
init_mac:
	brew install yt-dlp
check:
	cargo audit --ignore RUSTSEC-2020-0071 && cargo update && cargo outdated
build: check
	cargo build
run:
	cargo run
deploy: check
	cargo build --release && cargo semver bump patch && sudo -S cp target/release/yt-parallel /usr/local/bin/
build_container_arm:
	docker build --platform linux/x86_64 -t phiroict/yt-parallel:0.2.6 -f deploy/docker/Dockerfile_arm .
run_container_arm:
	docker run \
		--platform linux/x86_64 \
		-it \
		--mount type=bind,source=.,target=/home/phiro/mounts/Volume_1/youtube/ \
		--mount type=bind,source=$(shell pwd)/videolist.txt,target=/app/source/videolist.txt \
		phiroict/yt-parallel:0.2.6
build_linux_arm:
	cargo build --release --target x86_64-unknown-linux-gnu
all_container_arm: build_linux_arm build_container_arm run_container_arm