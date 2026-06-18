{ pkgs ? import <nixpkgs> {} }:

let fhs = pkgs.buildFHSEnv {
    name = "birdshit-fhs";
    targetPkgs = pkgs: with pkgs; [
      pnpm
      typescript
      nodejs

      rustc
      cargo
      cargo-watch
      rust-analyzer

      docker
      docker-compose
    ];

    # appending is useful in case of nested shells (unintended, bug)
    profile = ''
      export FHS_ENV="1$FHS_ENV"
    '';

    runScript = ''
      fish
    '';
  };


in {
  # use fhs.env to actually use it
  fhs = fhs,
  flake = flake,
}