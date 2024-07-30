{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";
  env.DATABASE_URL = "mysql://root:@localhost:3306/pyum";
  env.LD_LIBRARY_PATH = ".devenv/profile/lib/";

  dotenv.enable = true;

  # https://devenv.sh/packages/
  packages = [ pkgs.git pkgs.mysql pkgs.libmysqlclient ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "nightly";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/

  processes = {
    ping.exec = "ping localhost";
  };

  # https://devenv.sh/services/
  services.mysql = {
    enable = true;
    initialDatabases = [{ name = "pyum"; }];
    ensureUsers = [
      {
        name = "root";
        password = "";
        ensurePermissions = {
          "*" = "ALL PRIVILEGES";
        };
      }
    ];
  };
}
