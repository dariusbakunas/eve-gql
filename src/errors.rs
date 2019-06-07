#![recursion_limit = "1024"]

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
    }
}
