install:
	# uncomment the intended
	cargo install cargo-edit
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --feature derive
	cargo add chrono --feature serde
	cargo add env_logger
	cargo add dotenv
	cargo add jsonwebtoken
	cargo add argon2
	cargo add rand_core --feature "std"
	# sql cli

