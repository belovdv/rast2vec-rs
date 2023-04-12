workspace = out/1
mode = 
dir_source = data

all:
	cargo test

args = --mode ${mode} --workspace ${workspace} --dir-source ${dir_source}
run:
	cargo build
	RUST_BACKTRACE=1 time -p cargo run -- ${args}
