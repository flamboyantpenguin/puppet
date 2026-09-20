fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("desktop/windows/logo.ico");
        res.set_manifest_file("desktop/windows/manifest.xml");
        res.compile().unwrap();
    }
}
