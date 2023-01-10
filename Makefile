dev:
	docker-compose up -d

dev-down:
	docker-compose down

start-server:
	cargo watch -q -c -w src/ -x run

install:
	cargo add rocket@0.5.0-rc.2 --features json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add uuid --features v4
	# HotReload
	cargo install cargo-watch 