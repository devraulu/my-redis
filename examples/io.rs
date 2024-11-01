use tokio::{
    fs::File,
    io::{self, AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("/Users/visual/Downloads/marcas_workmate.csv").await?;
    let mut buffer = [0; 100];

    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes {:?}", &buffer[..n]);

    read_to_end_example().await?;
    write_all_example().await?;
    copy_example().await?;
    Ok(())
}

async fn read_to_end_example() -> io::Result<()> {
    let mut f = File::open("/Users/visual/Downloads/nerv.txt").await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;
    println!("Read the whole file {:?}", buffer);
    Ok(())
}

async fn write_example() -> io::Result<()> {
    let mut file = File::open("foo.txt").await?;

    let bytes = b"Hello my name is";
    let n = file.write(bytes).await?;
    println!("Wrote the first {:?} bytes of which amount to {}", bytes, n);

    Ok(())
}

async fn write_all_example() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    file.write_all(b"some bytes").await?;
    Ok(())
}

async fn copy_example() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;

    Ok(())
}
