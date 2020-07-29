use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Expect {0}, but None.")]
    None(String)
}

