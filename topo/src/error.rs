use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Csv(#[from] csv::Error),

    #[error("{0}")]
    Dxf(#[from] dxf::DxfError),

    #[error("{0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
