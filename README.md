# K(eep) I(t) S(imple) S(tupid) File Manager

## Goals

- become more proficient with rust
- test out the **rs-tui** package
- dog food a simple terminal file manager to use with tmux and or neovim

## Usage

- installation
    - `cargo install --git https://github.com/ethangumin/kissfm`
- run application
    - run `kfm` in the terminal
- view 'Quick Help' section while running application for usage commands
- make a config file if you'd like to set the editor in which to open files with 
    - application defaults to "vim"
    - create a file `kfm.toml` in your config directory, and set editor to anything, so long as it can be called from the terminal
    ```toml
    editor = "nvim"
