{ pkgs ? import <nixpkgs> {
    overlays = [ (import ./vespa-cli-overlay.nix) ];
    config = { allowUnfree = true; };
  }
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    just
    cargo
    tree
    poetry
    vespa-cli
    natscli
    nats-server
    ngrok
  ];
  shellHook = ''
    cat <<'EOF'
                .-.
                    `-'
        ___          LoRA
    .´   `'.     _...._
    :  LLAMA  :  .'      '.
    '._____.' /`(o)    (o)`\
    _|_______|/  :      :  \
    [_____________/ '------'  \
    /  o                    /
    `"`"|"`"`"`"`"`"`"`""=""===""`
    EOF

    echo "gm gm ⟁"
  '';
}
