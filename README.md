# sasquatchdev/confour
A connect four game featuring a virtual opponent written in rust

> [!Important]
> This repository is not actively being maintained, and might contain bugs, or other unknown issues.

## Table of Contents

- [Introduction](#introduction)
- [Known Issues](#known-issues)
- [Features](#features)
- [Installation & Usage](#installation--usage)

## Introduction

This project features a connect four game including a virtual opponent / bot. This opponent is implemented using a variant of the [minimax](https://en.wikipedia.org/wiki/Minimax) called ["negamax"](https://en.wikipedia.org/wiki/Negamax).

## Known Issues

- _(minor)_ _[algorithm]_ Sometimes, opponent does not (immediately) take winning move

## Features

- Play a 1v1 game of connect four against a bot
    - alpha-beta-pruning
    - transposition table
    - _(planned)_ move ordering
- _(planned)_ Play a 1v1 game against someone else (local multiplayer)

## Installation & Usage

To use this, you will need to have [git](https://git-scm.com/) and [rust](https://www.rust-lang.org/) installed. Next, clone this repository:

```bash
git clone https://github.com/sasquatchdev/confour.git && cd ./confour
```

Next, build and run the rust project (for better performance, use `--release`)

```bash
cargo build --release && ./target/release/confour(.exe)
```

or

```bash
cargo run --release
```
