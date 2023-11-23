@build input_file:
    # Command to compile the Rust file
    rustc -C opt-level=3 -C debuginfo=0 -C link-arg=/DEBUG:NONE -C lto "{{input_file}}.rs" -o "bin/{{input_file}}.exe"


