let
  sources = import ./npins;
  pkgs = import sources.nixpkgs {
    config.allowUnfree = true;
    overlays = [
        (import sources.rust-overlay)
    ];
  };
in
pkgs.mkShellNoCC {
  nativeBuildInputs = with pkgs; [
    clang
    wild
    (rust-bin.stable.latest.default.override {
      extensions = [
        "rust-src"
        "rust-analyzer"
      ];
    })
  ];
  shellHook = ''
    echo hi
  '';
}
