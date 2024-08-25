.PHONY: cargo_all_fmt run_api_server run_api_server_with_watch deno_task_start deno_test migrate_run migrate_run_refresh

cargo_all_fmt:
	cd ./src/backend && \
	cargo fmt && \
	cd ./submodules/migration && \
	cargo fmt && \
	cd ../oauth_gen && \
	cargo fmt && \
	cd ../model_entity && \
	cargo fmt

run_api_server:
	cd ./src/backend && \
	cargo run

run_api_server_with_watch:
	cd ./src/backend && \
	cargo watch -x run

deno_task_start:
	cd ./src/frontend && \
	deno task start

deno_test:
	cd ./src/frontend && \
	deno test

migrate_run:
	cd ./src/backend/submodules/migration && \
	cargo run

migrate_run_refresh:
	cd ./src/backend/submodules/migration && \
	cargo run refresh

