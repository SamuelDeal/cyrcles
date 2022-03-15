use which;

pub fn is_installed() -> bool {
    let result = which::which("gpg2");
    match result {
        Ok(_) => true,
        Err(which::Error::CannotFindBinaryPath) => false,
        Err(e) => panic!("{}", e),
    }
}
