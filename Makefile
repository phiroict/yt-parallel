clean:
	cargo clean
init:
	cargo install cargo-audit
	cargo install cargo-outdated
	cargo install cargo-update
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
	cargo build --release && sudo -S cp target/release/yt-parallel /usr/local/bin/

