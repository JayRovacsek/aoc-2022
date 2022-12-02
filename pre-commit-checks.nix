{ self }: {
  src = self;
  hooks = {
    nixfmt.enable = true;
    rustfmt.enable = true;
    cargo-check.enable = true;
  };
}
