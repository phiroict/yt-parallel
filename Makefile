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
	cargo install --force cargo-pkgbuild
init_fedora:
	sudo yum install gcc openssl openssl-devel yt-dlp python-pip -y 
init_arch:
	pacman -S which yt-dlp --needed
init_mac:
	brew install yt-dlp
check:
	cargo fmt && cargo audit && cargo update && cargo outdated
test:
	cargo test
build: check test
	cargo build
run:
	cargo run
run_win:
	cd target/debug && yt-parallel.exe -l "M:/Apps/videolist.txt"
version:
	cargo semver bump patch && cargo build --release
deploy: check test version
	sudo -S cp target/release/yt-parallel /usr/local/bin/ && git commit -am "Linux Release commit" && git tag v$(shell bash get_version_from_toml.sh)
deploy_win: check test version
	cmd /C  XCOPY target\release\yt-parallel.exe M:\Apps\ /y /q && git commit -am "Windows Release commit" && cmd /C python.exe get_version_from_toml.py
build_container_arm:
	docker build -t phiroict/yt-parallel:$(APP_VERSION) -f deploy/docker/Dockerfile_arm .
build_container_fedora_arm:
	podman build -t phiroict/yt-parallel:$(APP_VERSION) -f deploy/docker/Dockerfile_arm .
build_container_init:
	docker buildx build -t phiroict/yt-parallel-init:$(APP_VERSION) -f Dockerfile .
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
arch_package:
	cargo pkgbuild
publish:
	cargo publish
docker_build:
	docker build -t phiroict/yt-parallel:0.5.19 -f deploy/docker/Dockerfile .