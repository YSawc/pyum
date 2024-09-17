# pyum

Iot device manager controlling trigger value of each event.

## Run application

Before running application, up the nix process for database.

```sh
devenv up
```

After database process up, tun the backend and api with make command upon the shell of nix process.

- run the backend

```sh
devenv shell
make run_api_server_with_watch
```

- run the frontend

```sh
devenv shell
make deno_task_start
```

