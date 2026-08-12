fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/logo.ico");
        res.set_manifest_file("assets/windows/manifest.xml");
        res.compile().unwrap();
    }
}
