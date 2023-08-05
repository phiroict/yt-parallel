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

