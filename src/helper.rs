pub mod Helper {

    const DBG_STR: &str = "";
    const OK: i32 = 0;
    const ERR: i32 = 1;

    #[derive(Debug)]
    pub struct CLI {
        pub debug: bool,
        pub srcdir: Option<String>,
        pub ifile: Option<String>,
    }

    impl CLI {
        pub fn new() -> CLI {
            Self {
                debug: false,
                srcdir: None,
                ifile: None,
            }
        }

        pub fn Parse_Args(&mut self) {
            let clargs = std::env::args().collect::<Vec<String>>();
            if clargs.is_empty() {
                return;
            }
            for i in &clargs {
                if i == "-h" || i == "--help" || i == "-H" || i == "--Help" {
                    Help();
                } else if i == "-d" || i == "--debug" || i == "-D" || i == "--Debug" {
                    self.debug = true
                } else if i.starts_with("--src=") || i.starts_with("--SRC=") {
                    let idx = i.find("=").unwrap();
                    self.srcdir = Some(i[idx + 1..].to_string());
                } else if i.starts_with("--file=") || i.starts_with("--FILE=") {
                    let idx = i.find("=").unwrap();
                    self.ifile = Some(i[idx + 1..].to_string());
                } else {
                    Help();
                }
            }
        }
    }

    pub fn Help() {
        println!("{DBG_STR}");
        std::process::exit(OK);
    }
}
