{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }: {
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
