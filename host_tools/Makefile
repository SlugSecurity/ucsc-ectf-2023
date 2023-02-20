host_tools: FORCE
	mkdir -p /tmp/bld/host_tools
	cp -r /docker_env /tmp/bld/
	cp -r . /tmp/bld/host_tools
	cd /tmp/bld/host_tools && cargo build --release --out-dir ${TOOLS_OUT_DIR} -Z unstable-options

FORCE:;
