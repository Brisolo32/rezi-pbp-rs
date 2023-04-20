# Rezi-PBP-RS

A plugin for PBP that gets data from Rezi (but this time made using Rust!)

## How to use it:

Command Line: rezi-pbp-rs "query" "cache location"

PBP: Install the plugin and select it on the search page

## Building:

You will need:

- Rust

First, make sure you have Rust installed, if not. [Download it](https://rustup.rs)

Then, run `cargo build --release` to build it

## How it works:

It makes an request to Rezi's API and gets the data from it, after that it just writes to an file named `results.json`

## Why?:

This has been made to use with PBP although you can use it with another program since it outputs to an json file. I have not made Rezi. [Wamy-Dev has](https://github.com/Wamy-Dev)

Thanks to:

- Wamy-Dev for making Rezi!
