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
      }
    );
}
