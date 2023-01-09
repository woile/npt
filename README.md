# NPT

> Nix Package Tool

A (humble) successor to linux's `apt`, which makes life easier when using [nix](https://nixos.org/) as a package manager.

This is a tool I'm using in my nix journey, it's supposed to ease the transition from other
package managers to nix.

The idea is to use [nix profiles](https://nixos.org/manual/nix/stable/package-management/profiles.html) in the way you'd use `apt` or `brew`.

It is a work in progress, but feel free to play with it.

## Requirements

Make sure you have installed [nix the package manager](https://nixos.org/download.html).

## Goals

- Make it easy to transition from traditional package managers (`apt`, `brew`, `pacman`, etc)
- Good and ituitive UX
- Learn nix while using, this can display the commands executed (wip)

## Installation

```sh
nix profile install 'github:woile/npt#npt'
```

## Usage

```$ npt --help
Nix Package Tool

Usage: npt <COMMAND>

Commands:
  install (i)  Install packages for the profile, if no repository provided, it defaults to nixpkgs
  list (ls)    List installed packages
  search (s)   Find a package in the registry, if no repository provided, it defaults to nixpkgs
  remove (rm)  Remove one or more packages
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## TODO

- [ ] Accept a package or expression in the List command
- [ ] Improve help sections
- [ ] Implement `update` command
- [ ] Try to implement `shell` command
- [ ] provide compiled tar's to speed up flakes

## Contributing

