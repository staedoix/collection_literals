{ inputs, ... }:
{
  perSystem =
    {
      inputs',
      pkgs,
      ...
    }:
    let
      fenix = inputs'.fenix.packages;
      crane-lib = (inputs.crane.mkLib pkgs).overrideToolchain fenix.stable.toolchain;
    in
    {
      _module.args = {
        inherit fenix crane-lib;
      };
    };
}
