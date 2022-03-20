{
  perSystem =
    { fenix, self', ... }:
    {
      pre-commit.settings.hooks = {
        treefmt = {
          enable = true;
          package = self'.formatter;
        };
        statix.enable = true;
        deadnix.enable = true;
        taplo.enable = true;
        clippy = {
          enable = true;
          package = fenix.stable.clippy;
          after = [
            "taplo"
            "treefmt"
            "statix"
            "deadnix"
          ];
          settings.denyWarnings = true;
        };
      };
    };
}
