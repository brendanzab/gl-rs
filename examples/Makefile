all: build

build:
	(cd static_basic && cargo build)
	(cd static_triangle && cargo build)
#	error: couldn't open file (no such file or directory (No such file or directory); path=deps/khronos-api/gl.xml; mode=open; access=read)
#	(cd struct_triangle && cargo build)

update:
	(cd static_basic && cargo update)
	(cd static_triangle && cargo update)
#	(cd struct_triangle && cargo update)

clean:
	(cd static_basic && cargo clean)
	(cd static_triangle && cargo clean)
#	(cd struct_triangle && cargo clean)
