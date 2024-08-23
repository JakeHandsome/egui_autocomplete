{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        markdownFilter = path: _type: builtins.match ".*md$" path != null;
        markdownOrCargo = path: type: (markdownFilter path type) || (craneLib.filterCargoSources path type);
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = markdownOrCargo;
          name = "source";
        };
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          makeWrapper
        ];
        buildInputs = with pkgs; [
          # misc. libraries
          openssl

          # GUI libs
          libxkbcommon
          libGL
          fontconfig

          # wayland libraries
          wayland

          # x11 libraries
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
        ];
        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        demo = craneLib.buildPackage (
          commonArgs
          // {
            pname = "demo";
            cargoExtraArgs = "-p demo";
            postInstall = ''
                wrapProgram $out/bin/demo \
                    --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs} 
            '';
          }
        );
      in
      with pkgs;
      {
        packages = {
          inherit bin demo;
          default = bin;
        };
        devShells.default = mkShell {
          # Get all the inputs to build our flake
          inputsFrom = [ bin ];
          buildInputs = with pkgs; [ ];
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
