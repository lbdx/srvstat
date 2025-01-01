{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage {
  pname = "srvstat";
  version = "0.1";
  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };
  src = pkgs.lib.cleanSource ./.;
  nativeBuildInputs = [ pkgs.pkg-config pkgs.cmake ];
  buildInputs = [ pkgs.openssl ];
}

