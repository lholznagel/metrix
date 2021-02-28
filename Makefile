.PHONY: docs docs-open sync-virgo

docs:
	cargo doc --no-deps --document-private-items --all-features

docs-open:
	cargo doc --no-deps --document-private-items --all-features --open

musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-server: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/metrix_server /opt/metrix/metrix_server
	sudo systemctl restart metrix_server

deploy-db: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/metrix_db /opt/metrix/metrix_db
	sudo mkdir -p /var/metrix/db/storage
	sudo systemctl restart metrix_db

deploy:
	make deploy-db
	make deploy-server

sync-virgo:
	rsync --recursive --inplace --delete --quiet --exclude={'.git','target','web/node_modules'} . virgo:dev/metrix
