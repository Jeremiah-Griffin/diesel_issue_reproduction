set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
export RUSTFLAGS := "--cfg uuid_unstable"

alias b := build
build: regen
    cargo build

alias c := check
check: regen
    cargo check


alias f := fix_fmt
fix_fmt: regen
    cargo fix
    cargo fmt

alias i := init
init: update
    cargo check
    cargo build

lint: regen
  cargo clippy

alias r := run
run: regen
    cargo run

regen:
    diesel database reset

release: regen
  cargo build --release

alias u:= update
update: regen
    cargo update





