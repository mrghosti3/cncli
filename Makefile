RUST_CMD=cargo run --

run_line:
	${RUST_CMD} ./data/line.dxf

run_line_diff:
	${RUST_CMD} ./data/line.dxf | diff - ./data/line.gcode

run_triangle:
	${RUST_CMD} ./data/triangle.dxf | diff - ./data/triangle.gcode

test_line:
	${RUST_CMD} ./data/line.dxf | diff -q - ./data/line.gcode

test_tri:
	${RUST_CMD} ./data/triangle.dxf | diff -q - ./data/triangle.gcode

tests: test_line test_tri
