workspace = out/1
dir_source = data

all:
	cargo test

args = --workspace ${workspace} --dir-source ${dir_source} -t
run:
	rm -rf out/1/*
	cargo build
	RUST_BACKTRACE=1 time -p cargo run -- ${args}

img = 
clean_svg:
	scour -i examples/${img}.svg -o examples/${img}_s.svg 

