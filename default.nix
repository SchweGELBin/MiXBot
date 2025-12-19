{
  fenix ? import (fetchTarball {
    url = "https://github.com/nix-community/fenix/archive/f1526533e3a59a666dbae99594c9d29b201f302d.tar.gz";
    sha256 = "0gw1095ss53pfqhniq2wvg8wr340k830wpbjlmaq5jga18vkp21q";
  }) { },
  lib,
  pkgs,
}:

let
  rustToolchain =
    with fenix;
    combine [
      minimal.toolchain
      targets.aarch64-unknown-linux-gnu.minimal.rust-std
      targets.x86_64-unknown-linux-gnu.minimal.rust-std
    ];
  rustNightly = pkgs.makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };
in

rustNightly.buildRustPackage (finalAttrs: {
  pname = "mixbot";
  version = "0.3.1";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "Minecraft MiXBot";
    homepage = "https://github.com/SchweGELBin/MiXBot";
    changelog = "https://github.com/SchweGELBin/MiXBot/blob/v${finalAttrs.version}/docs/CHANGELOG.md";
    license = lib.licenses.mit;
    mainProgram = finalAttrs.pname;
    maintainers = with lib.maintainers; [ SchweGELBin ];
  };
})
