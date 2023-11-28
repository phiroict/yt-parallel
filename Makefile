APP_VERSION ?= $(shell bash get_version_from_toml.sh)
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
run_win:
	cd target/debug && yt-parallel.exe -l "c:/Users/phiro/Desktop/videolist.txt"
deploy: check
	cargo semver bump patch && cargo build --release &&  sudo -S cp target/release/yt-parallel /usr/local/bin/ && git commit -am "Linux Release commit" && git tag v$(shell bash get_version_from_toml.sh)
deploy_win: check
	cargo semver bump patch && cargo build --release &&  cmd /C  XCOPY target\release\yt-parallel.exe I:\Apps\ /y /q && git commit -am "Windows Release commit" && cmd /C python.exe get_version_from_toml.py
build_container_arm:
	docker build -t phiroict/yt-parallel:$(APP_VERSION) -f deploy/docker/Dockerfile_arm .
run_container_arm:
	-docker rm yt-parallel
	docker run \
		--rm  \
		--name yt-parallel \
		--platform linux/x86_64 \
		-it \
		--mount type=bind,source=.,target=/home/phiro/mounts/Volume_1/youtube/ \
		--mount type=bind,source=$(shell pwd)/videolist.txt,target=/app/source/videolist.txt \
		phiroict/yt-parallel:$(APP_VERSION)
build_linux_arm:
	cargo build --release --target x86_64-unknown-linux-gnu
all_container_arm: build_linux_arm build_container_arm run_container_arm
push_container:
	docker push phiroict/yt-parallel:$(APP_VERSION)
deploy_container: build_linux_arm build_container_arm push_container

test:
	cmd /C  XCOPY target\release\yt-parallel.exe I:\Apps\ /y /q