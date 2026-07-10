{
  description = "birdshit dev env";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/0ad6f47ea4fe188f4bc8f0380f93ae8523337c6c";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            cargo-watch
            rust-analyzer
            rustPlatform.rustLibSrc
            pkg-config
            nodejs_24
            pnpm
            postman
            playwright-driver.browsers
            terraform terraform-ls
          ];

          shellHook = ''
            export RUST_SRC_PATH="$(rustc --print target-libdir)/src"
            export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}
            export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true
          '';
        };
      }
    );
}