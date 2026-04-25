pkgs:

pkgs.mkShell {
  name = "ts2wasm";

  packages =
    (with pkgs; [
      rustc
      cargo
      rustfmt
      clippy
      cargo-nextest
      nodejs
      git
      wamr
      ripgrep
      jq
      mold
    ])
    ++ [
      # Hyphenated pname; `with pkgs; [ ast-grep ]` would parse as subtraction.
      pkgs."ast-grep"
    ];
}
