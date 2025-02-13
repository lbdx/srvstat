{ pkgs, lib, config, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.cmake
    pkgs.openssl
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping localhost";

  # See full reference at https://devenv.sh/reference/options/
  enterShell = "\n    git --version\n    rustc --version\n  ";

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };

  env.BROKER_URL = "tcp://localhost:1883";
}
