target/debug/main:
	cargo build

.PHONY: build-container
build-container: target/debug/main
	podman build -t cloudflare-ddns --network=host -f build/container/Containerfile .
