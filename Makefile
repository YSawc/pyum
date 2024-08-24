.PHONY: run_api_server run_api_server_with_watch deno_task_start deno_test

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
