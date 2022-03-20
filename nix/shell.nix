{
  perSystem =
    {
      config,
      self',
      pkgs,
      fenix,
      crane-lib,
      ...
    }:
    {
      devShells.default = crane-lib.devShell {
        inputsFrom = [
          self'.packages.default
        ];

        shellHook = # bash
          ''
            ${config.pre-commit.installationScript}
          '';

        packages = [
          pkgs.gitFull
          pkgs.cargo-release
          fenix.stable.rust-analyzer
        ];
      };
    };
}
