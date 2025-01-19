use std::fs;
use tokio::fs as tokio_fs;
use tokio::io::{AsyncReadExt, AsyncSeekExt, Result, SeekFrom};

#[tokio::main]
async fn main() -> Result<()> {
    // READ
    // std fs
    let read = fs::read("./README.md")?;
    println!("STD: {:?}", String::from_utf8_lossy(&read));

    // Equivalent to above using Tokio fs
    let read = tokio_fs::read("./README.md").await?;
    println!("TOKIO: {:?}", String::from_utf8_lossy(&read));

    // More flexible way to read
    // NOTE: Rust uses internal cursor to read a file this way.
    let mut file = tokio_fs::File::open("./README.md").await?;

    // The first 10 bytes is already read. So the cursor advances to 10. You can reset the cursor using using command shown below.
    let mut buffer = [0; 10];
    let _ = file.read(&mut buffer).await?;
    println!("First 10 bytes: {:?}", buffer);

    // Uncomment this to reset the cursor
    // file.seek(SeekFrom::Start(0)).await?;

    // Since, the cursor starts from 10 and stops at end, your content will be cut off by 10 bytes at first.
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer).await?;
    println!("All bytes: {:?}", buffer);

    // Uncomment this to reset the cursor
    // file.seek(SeekFrom::Start(0)).await?;

    // Since, all content of file has been read, this will return empty.
    let mut str = String::new();
    let _ = file.read_to_string(&mut str).await?;

    println!("As String: {:?}", str);

    // Write
    let mut file = tokio_fs::File::create("test.txt").await?;

    let size = file.write(b"Hello Rust!").await?;

    println!("Written bytes {}", size);

    // Copy
    let mut file = tokio_fs::File::open("./README.md").await?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;

    let mut file2 = tokio_fs::File::create("./test.txt").await?;

    let mut buf: &[u8] = &buf;

    tokio::io::copy(&mut buf, &mut file2).await?;

    Ok(())
}
