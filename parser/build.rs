extern crate cc;

fn main() {
    cc::Build::new().file("http-parser/http_parser.c").file("src/c_extension.c").warnings(false).compile("http_parser");
}
