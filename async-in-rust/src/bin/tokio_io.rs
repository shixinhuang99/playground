use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    write_file().await?;
    read_file().await?;

    Ok(())
}

async fn write_file() -> Result<()> {
    let mut file = File::create("foo.txt").await?;
    file.write(b"some bytes").await?;
    Ok(())
}

async fn read_file() -> Result<()> {
    let mut file = File::open("foo.txt").await?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).await?;
    let contents = String::from_utf8_lossy(&buf);

    println!("{}", contents);
    Ok(())
}
