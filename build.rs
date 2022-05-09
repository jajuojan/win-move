fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("win-move.exe.manifest");
        res.compile().unwrap();
    }
}
