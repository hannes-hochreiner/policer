export def refresh_flake [] {
  podman run --rm -it -v $"($env.PWD):/workspace:z" nixos/nix bash -c "nix build --extra-experimental-features nix-command --extra-experimental-features flakes --recreate-lock-file /workspace"
}
