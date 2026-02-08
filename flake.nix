{
  description = "Rust development environment for Neovim";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-darwin"; # Apple Silicon Mac
      # system = "x86_64-darwin"; # Intel Mac
      # system = "x86_64-linux"; # Linux
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer
          clippy
          rustfmt
        ];

        shellHook = ''
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          echo "🦀 Rust Development Environment Loaded!"
          echo "✅ Ready for Neovim (rust-analyzer, clippy, rustfmt)"
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          rustc --version
          cargo --version
        '';
      };
    };
}
