#[cfg(test)]
mod python_api {
    #[test]
    pub fn test_py() {
        // run `test.py`
        let output = std::process::Command::new("python")
            .arg(format!(
                "{}",
                std::env::current_dir().unwrap_or_default().display()
            ))
            .output()
            .unwrap();
        assert!(output.status.success(), "status({:?}) with stderr: \n{:#?}\n", output.status, String::from_utf8_lossy(&output.stderr));
    }
}
