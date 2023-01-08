# NPT

> Nix Package Tool

A (humble) successor to linux's `apt`, which makes life easier when using [nix](https://nixos.org/) as a package manager.

This is a tool I'm using in my nix journey, it's supposed to make the transition from other
package managers to nix.

It is not production ready, but you can play with it.

## Requirements

Make sure you have installed [nix the package manager](https://nixos.org/download.html).

## Goals

- Make it easy to transition from traditional package managers (`apt`, `brew`, `pacman`, etc)
- Good and ituitive UX
- Learn nix while using, this can display the commands executed.

## Installation

```sh
nix profile install 'github:woile/npt' --no-write-lock-file
```

## Usage

```$ npt --help
Nix Package Tool

Usage: npt <COMMAND>

Commands:
  install  Install packages for the profile, if no repository provided, it defaults to nixpkgs
  update   Update all or specific packages
  search   Find a package in the registry, if no repository provided, it defaults to nixpkgs
  remove   Remove one or more packages
  shell    Open a shell with the given packages
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## TODO

- [ ] implement `update` command
- [ ] implement `shell` command
- [ ] provide compiled tar's to speed up flakes

## Contributing

