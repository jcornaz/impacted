{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
  };

  outputs = { self, nixpkgs, ... }: {
    packages = builtins.mapAttrs (system: pkgs: {
      default = pkgs.callPackage ({ rustPlatform, debug ? false }:
        rustPlatform.buildRustPackage {
          name = "countdown";
          buildMode = if debug then "debug" else "release";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        }
      ) {};
      debug = self.packages.${system}.default.override { debug = true; };
    }) nixpkgs.legacyPackages;

    devShells = builtins.mapAttrs (system: pkgs: {
      default = pkgs.mkShell {
        packages = [
          pkgs.just
          pkgs.rustc
          pkgs.rust-analyzer
          pkgs.rustfmt
          pkgs.clippy
          pkgs.cargo
          pkgs.cargo-hack
          pkgs.cargo-msrv
        ];
        shellHook = ''
          cargo --version
        '';
      };
    }) nixpkgs.legacyPackages;
  };
}
