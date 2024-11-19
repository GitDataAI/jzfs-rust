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
    fn mkdir(&mut self, local: &str) -> Result<(), Self::Error>;
    fn rmdir(&mut self, local: &str) -> Result<(), Self::Error>;
    fn move_files(&mut self, local: &str, remote: &str) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}

#[test]
fn build(){
    println!("Hello FileTree")
}