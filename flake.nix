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
      pname = "noematic";
      version = "0.1.0";
      src = self;
      npmRoot = self;
      mkExt =
        pkgs:
        pkgs.stdenvNoCC.mkDerivation {
          pname = "${pname}-extension";
          inherit version src;

          installFlags = [ "DESTDIR=${placeholder "out"}" ];
        };
      mkNoematic =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          inherit pname version src;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      overlay = final: prev: {
        noematic = mkNoematic final;
        noematic-static = mkNoematic final.pkgsStatic;
        noematic-extension = mkExt final;
      };
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ overlay ];
        };
      in
      {
        packages = {
          noematic = pkgs.noematic;
          noematic-static = pkgs.noematic-static;
          noematic-extension = pkgs.noematic-extension;
          all = pkgs.symlinkJoin {
            name = "noematic-all";
            paths = with pkgs; [
              noematic
              noematic-extension
            ];
          };
          default = self.packages.${system}.all;
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            pkgs.noematic
          ];
          packages = with pkgs; [
            rust-analyzer
            rustfmt
            clippy
            cargo-deny
            importNpmLock.hooks.linkNodeModulesHook
            nodejs
            playwright-driver.browsers
          ];
          npmDeps = pkgs.importNpmLock.buildNodeModules {
            inherit npmRoot;
            inherit (pkgs) nodejs;
          };
          PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers}";
          PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = true;
        };
      }
    );
}
