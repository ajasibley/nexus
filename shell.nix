{ pkgs ? import <nixpkgs> {
    overlays = [ (import ./overlays/vespa-cli-overlay.nix) ];
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
    ipfs
    ngrok
    # openssl_1_1 <- curently openssl needs to be installed with homebrew on mac
  ];
  shellHook = ''
    cat << 'EOF'
    .-.    n=69 *  *  *    *         * * *
    `-'     *  *      *  *  *        *   *
    __ * *._* *_: * * *:*  *:*.*   *   * *
    . ́ * *`*. * *.:  * * *  *::. .-*
    : * * *:*     *  :* * * *  *:'*
    .'* * * * ~:* *.:*`*(*)*`-.:' *
    _____________________
    |  _______________  |
    | | *  *  *  *  * | |
    |_|___________________|
    |   | |               |
    |___| |_             _|
    |   |_|   |     o    | |
    |     |   | [][]-O-|[]|
    |_____|___|__(_)__ (_)|___
    |   o   _|_  |  .  |  _|
    |      (___)_|(___)_| (_)
    |_o__o_o__o_o__o_o__o_o__
    |  .   .   .   .   .  | .
    |_|_|_|_|_|_|___|_|_|_|___|
    |   .   .   .   .   |
    _|___|___|___|___|___|____
    o o o o o o o o o o o o o

    gm gm ⟁
    EOF
  '';
}
