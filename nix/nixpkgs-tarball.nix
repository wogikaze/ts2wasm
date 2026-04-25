# Pinned nixpkgs (nixpkgs-unstable @ 01fbdeef22b76df85ea168fbfe1bfd9e63681b30).
# To bump: pick a commit on https://github.com/NixOS/nixpkgs/commits/nixpkgs-unstable
# then run (hash is for the *unpacked* tree, not raw .tar.gz bytes):
#   nix-prefetch-url --unpack "https://github.com/NixOS/nixpkgs/archive/<rev>.tar.gz"
# or run once without sha256 and paste the `got:` hash from the mismatch error.
builtins.fetchTarball {
  url = "https://github.com/NixOS/nixpkgs/archive/01fbdeef22b76df85ea168fbfe1bfd9e63681b30.tar.gz";
  sha256 = "0b76m4i1sn0dg78ylapvbkgw9knkf6lm1lss39w6zyshgv1rbi0q";
}
