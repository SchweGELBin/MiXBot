{
  fenix ? import (fetchTarball {
    url = "https://github.com/nix-community/fenix/archive/47c5355eaba0b08836e720d5d545c8ea1e1783db.tar.gz";
    sha256 = "0a52hq3nlacbzzkqfdszyg1syygypk1q7cljfrr134mqnfrqma3i";
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
  version = "1.0.0";

  src = ./.;
  cargoLock = {
    allowBuiltinFetchGit = true;
    lockFile = ./Cargo.lock;
  };

  meta = {
    description = "Minecraft MiXBot";
    homepage = "https://github.com/SchweGELBin/MiXBot";
    changelog = "https://github.com/SchweGELBin/MiXBot/blob/v${finalAttrs.version}/docs/CHANGELOG.md";
    license = lib.licenses.mit;
    mainProgram = finalAttrs.pname;
    maintainers = [ lib.maintainers.SchweGELBin ];
  };
})
