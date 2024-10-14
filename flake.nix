{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    let
      makeNoematic =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          name = "noematic";
          pname = "noematic";
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildNoDefaultFeatures = true;
          buildInputs = with pkgs; [ sqlite ];
          src = builtins.path {
            path = ./.;
            name = "noematic-src";
          };
          postBuild = ''
            make DESTDIR=${placeholder "out"}
          '';
        };
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.noematic = makeNoematic pkgs;
        packages.default = self.packages.${system}.noematic;
      }
    );
}
