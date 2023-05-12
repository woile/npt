# NPT

> Nix Package Tool

A (humble) successor to linux's `apt`, which makes life easier when using [nix](https://nixos.org/) as a package manager.

This is a tool I'm using in my nix journey, it's supposed to ease the transition from other
package managers to nix.

The idea is to use [nix profiles](https://nixos.org/manual/nix/stable/package-management/profiles.html) in the way you'd use `apt` or `brew`.

It is a work in progress, but feel free to play with it.

## Requirements

- Install [nix the package manager](https://nixos.org/download.html)
- [Enable flakes](https://nixos.wiki/wiki/Flakes#Enable_flakes)


## Goals

- Help the transition from traditional package managers (`apt`, `brew`, `pacman`, etc) to nix
- Good and intuitive UX
- Learn nix while using, this can display the commands executed `npt --teacher install <package>`

## Installation

```sh
nix profile install 'github:woile/npt#npt'
```

## Usage

```$ npt --help
Nix Package Tool

Usage: npt [OPTIONS] <COMMAND>

Commands:
  install  Install packages for the profile, if no repository provided, it defaults to nixpkgs [aliases: i]
  list     List installed packages [aliases: ls]
  upgrade  Update all or specific packages [aliases: u]
  search   Find a package in the registry, if no repository provided, it defaults to nixpkgs [aliases: s]
  remove   Remove one or more packages [aliases: rm]
  help     Print this message or the help of the given subcommand(s)

Options:
  -t, --teacher
  -h, --help     Print help information
  -V, --version  Print version information
```

## TODO

- [ ] How to make search accept a repo only?
- [ ] How to make search accept a regex?
- [ ] Accept a package or expression in the List command
- [ ] Implement `update` command
- [ ] provide compiled tar's to speed up flakes

## Contributing

```sh
nix flake update
```
