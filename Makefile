start: ##@cli Help
	cargo run -- help
format: ##@format
	cargo fmt --quiet
lint: ##@lint the project
	cargo clippy --quiet
