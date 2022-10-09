extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./src/bin/res/flc_icon.ico");
        res.compile().unwrap();
    }
}
