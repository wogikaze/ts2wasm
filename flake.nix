{
  description = "ts2wasm dev shell (nixpkgs pinned via builtins.fetchTarball in nix/nixpkgs-tarball.nix)";

  outputs =
    { self }:
    let
      tarball = import ./nix/nixpkgs-tarball.nix;
      lib = (import tarball { system = "x86_64-linux"; }).lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    in
    {
      devShells = lib.genAttrs systems (
        system:
        let
          pkgs = import tarball { inherit system; };
        in
        {
          default = import ./nix/devshell.nix pkgs;
        }
      );
    };
}
