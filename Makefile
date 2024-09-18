.PHONY: cargo_all_fmt run_api_server run_api_server_verbose run_api_server_with_watch deno_task_start deno_test migrate migrate_rollback_last migrate_refresh create_migrate- deno_add

cargo_all_fmt:
	cd ./src/backend && \
	cargo fmt && \
	cd ./packages/migration && \
	cargo fmt && \
	cd ../oauth_gen && \
	cargo fmt && \
	cd ../model_entity && \
	cargo fmt

run_api_server:
	cd ./src/backend && \
	cargo run

run_api_server_verbose:
	cd ./src/backend && \
	cargo run --verbose

run_api_server_with_watch:
	cd ./src/backend && \
	cargo watch -x run

deno_task_start:
	cd ./src/frontend && \
	deno task start

deno_test:
	cd ./src/frontend && \
	deno test

migrate:
	cd ./src/backend/packages/migration && \
	cargo run

migrate_rollback_last:
	cd ./src/backend/packages/migration && \
	cargo run -- down

migrate_refresh:
	cd ./src/backend/packages/migration && \
	cargo run refresh

create_migrate-: $(addprefix create_migrate-, $(Filename))

create_migrate-%:
	cd ./src/backend/packages/migration && \
	cargo run -- generate ${@:create_migrate-%=%}

deno_add-: $(addprefix deno_add-, $(Package))

deno_add-%:
	cd ./src/frontend && \
	deno add ${@:deno_add-%=%}
