{ inputs, ... }:
{
  perSystem =
    {
      config,
      self',
      pkgs,
      lib,
      ...
    }:
    {
      devShells.default = pkgs.mkShell {
        name = "redirectarr-shell";
        inputsFrom = [
          config.treefmt.build.devShell
          config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
        ];
        packages = with pkgs; [
          # nix stuff
          nixd
          nixfmt

          # rust stuff
          rustfmt
          clippy
          bacon

          # utilities
          gitflow
          just
          lazygit

          # dotfiles programs
          inputs.dotfiles.packages.${system}.quick-results
          inputs.dotfiles.packages.${system}.upjust
          # inputs.dotfiles.packages.${system}.cargo-update
        ];
        shellHook = ''
          echo "welcome to the rust development environment for the redirectarr package" | ${pkgs.cowsay}/bin/cowsay | ${pkgs.lolcat}/bin/lolcat 2> /dev/null;
        '';
      };
    };
}
