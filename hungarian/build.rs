fn main() {
    cc::Build::new()
        .file("src/hungarian.c")
        .compile("hungarian_c_lib");
}
