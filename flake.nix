{
  inputs = {
  fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        defaultPackage = (pkgs.makeRustPlatform {
          inherit (fenix.packages.${system}.minimal) cargo rustc;
        }).buildRustPackage rec {
          pname = "coolttviz";
          version = "0.1.0";
          src = ./.;
          cargoSha256 = "5z+I5JTkCIqXwV3S7V1y3+b5xUaYY4SICi1xMA6lf/M=";
          buildInputs = with pkgs; [ 
            xorg.libX11 
            xorg.libX11.dev 
            xorg.libXcursor
            xorg.libXcursor.dev
            xorg.libXrandr
            xorg.libXrandr.dev
            xorg.libXi
            xorg.libXi.dev
            libGL
          ];
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        };
      });
}
