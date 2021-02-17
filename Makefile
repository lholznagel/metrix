.PHONY: docs docs-open sync-virgo

docs:
	cargo doc --no-deps --document-private-items --all-features

docs-open:
	cargo doc --no-deps --document-private-items --all-features --open

deploy-server: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/metrix_server /opt/caph/metrix_server
	sudo systemctl restart metrix_server

deploy-db: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/metrix_db /opt/caph/metrix_db
	sudo mkdir -p /var/caph/db/storage
	sudo systemctl restart metrix_db

sync-virgo:
	rsync --recursive --update --inplace --delete --quiet --exclude={'.git','target','web/node_modules'} . virgo:dev/metrix
