{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          libXcursor
          libX11
          libXi
          libXrandr
          libxkbcommon
          libxcb
        ];

        # This makes the dynamically loaded libraries discoverable at runtime
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

        shellHook = ''
          export PATH="$HOME/.cargo/bin:$PATH"
        '';
      };
    };
}
