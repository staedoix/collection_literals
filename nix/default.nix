{
  imports = [
    ./rust.nix
    ./shell.nix
    ./pre-commit.nix
    ./treefmt.nix
  ];

  perSystem =
    {
      crane-lib,
      pkgs,
      lib,
      ...
    }:
    let
      crate = {
        src = crane-lib.cleanCargoSource ../.;
        buildInputs = lib.optional pkgs.stdenv.isDarwin pkgs.libiconv;
      };

      deps = {
        cargoArtifacts = crane-lib.buildDepsOnly crate;
      };

      collection_literals = crane-lib.buildPackage (crate // deps);
    in
    {
      packages = {
        default = collection_literals;
        doc = crane-lib.cargoDoc (crate // deps);
      };
      checks = {
        inherit collection_literals;
      };
    };
}
