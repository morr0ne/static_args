fn main() {
    #[cfg(unix)]
    for arg in static_args::static_args() {
        println!("{}", String::from_utf8_lossy(arg.to_bytes()))
    }

    #[cfg(windows)]
    for arg in static_args::static_args_windows() {
        println!("{}", String::from_utf16_lossy(arg))
    }
}
