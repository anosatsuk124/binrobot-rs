pub fn wait_for_starting() {
    loop {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("Couldn't read_line");
        if s.as_str().trim_end() == "STARTED" {
            break;
        }
    }
}
