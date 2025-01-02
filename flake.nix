{
  inputs = {
    nixpkgs.url = "nixpkgs";
    rustoverlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rustoverlay,
      utils,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rustoverlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        toolchain = pkgs.rust-bin.stable.latest;
      in
      {
        devShell = pkgs.mkShell {
          name = "devshell";
          buildInputs = [ toolchain.default ];
        };

        overlays.default = final: prev: {
          xtee = final.rustPlatform.buildRustPackage {
            pname = "xtee";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };

        packages.default =
          (import nixpkgs {
            inherit system;
            overlays = [ self.overlays.${system}.default ];
          }).xtee;
      }
    );
}
