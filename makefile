workspace = out/1
mode = 
dir_source = data

all:
	cargo test

args = --workspace ${workspace} --dir-source ${dir_source} -t
run:
	rm -rf out/1/*
	cargo build
	RUST_BACKTRACE=1 time -p cargo run -- ${args}
