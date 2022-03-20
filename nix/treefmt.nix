{
  perSystem =
    {
      fenix,
      pkgs,
      lib,
      ...
    }:
    let
      rustfmt-config =
        pkgs.writeText "rustfmt.toml" # toml
          ''
            edition = "2021"
            match_arm_leading_pipes = "Preserve"
            struct_lit_width = 0
            struct_variant_width = 0
            single_line_let_else_max_width = 0
            single_line_if_else_max_width = 0
            use_field_init_shorthand = true
            chain_width = 30
          '';

      treefmt = {
        projectRootFile = "flake.nix";
        programs = {
          nixfmt.enable = true;
          taplo.enable = true;
        };
        settings.formatter.rustfmt = {
          command = lib.getExe' fenix.stable.rustfmt "rustfmt";
          options = [
            "--config-path"
            "${rustfmt-config}"
          ];
          includes = [
            "**.rs"
          ];
        };
      };
    in
    {
      inherit treefmt;
    };
}
