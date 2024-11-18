use async_trait::async_trait;

pub mod location;


#[async_trait]
pub trait AsyncFile{
    type Input;
    type Output;
    type Error;

    async fn read(&mut self) -> Result<Self::Input, Self::Error>;
    async fn write(&mut self, input: &Self::Input) -> Result<Self::Output, Self::Error>;
}

pub trait AsyncFolder{
    type Error;
    async fn mkdir(&mut self, local: &str) -> Result<(), Self::Error>;
    async fn rmdir(&mut self, local: &str) -> Result<(), Self::Error>;
    async fn move_files(&mut self, local: &str, remote: &str) -> Result<(), Self::Error>;
}