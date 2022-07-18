extern crate cc;

fn main() {

    cc::Build::new()
        .file("src/foo.c")
        .flag("-fsanitize=address")
        .compile("libfoo.a");
}
