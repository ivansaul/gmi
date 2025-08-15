use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("csv error {0}")]
    Csv(#[from] csv::Error),

    #[error("dxf error {0}")]
    Dxf(#[from] dxf::DxfError),

    #[error("Io error {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
