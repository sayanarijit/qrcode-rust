use std::env;
use std::io::Read;

pub fn main() {
    let bytes = if let Some(arg) = env::args().nth(1) {
        arg.as_bytes().to_vec()
    } else {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let mut bytes = Vec::new();
        stdin.read_to_end(&mut bytes).expect("Failed to read from stdin");
        bytes
    };

    let code = qrencode::QrCode::new(bytes).unwrap();

    println!("{}", code.render().dark_color("\x1b[7m  \x1b[0m").light_color("\x1b[49m  \x1b[0m").build());
}
