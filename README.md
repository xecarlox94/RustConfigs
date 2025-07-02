# Rust_configs


Simple bash templating program to run docker in a sane manner.


## Requirements

You have to install the Nix package manager and enable the "flake" and "commands" experimental features.



[Install nix and enable nix flake](https://dev.to/arnu515/getting-started-with-nix-and-nix-flakes-mml)



## Test instalation


Please run this before running anyother command!


```bash
nix run github:xecarlox94/RustConfigs -- --help
```

## Example Run Program


Common example command to bootstrap a simple program.


```bash
nix run github:xecarlox94/RustConfigs -- --project new --base_image ubuntu -n -d -x
```

To run the docker container

```bash
cd new && ./run.sh
```
