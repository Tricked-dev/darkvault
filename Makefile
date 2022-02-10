build:
	yarn build
	cargo build --release
dev:
	echo make sure you have a .env.development with the url of the backend and use the cors firefox extension
	cargo watch -x run & yarn dev
