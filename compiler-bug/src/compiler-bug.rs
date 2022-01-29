use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("The following demonstrates what appears to be a compiler bug in rust stable 1.57:");
    println!("when compiled & run in release mode, this program works as expected");
    println!("(issuing the file not found message); however, when compiled & run in");
    println!("debug mode, the process sigfaults with 'stack exceeded' error.\n");

    let file_path = "non-existing-file.txt";
    let file_reader = async_std::fs::File::open(file_path).await
        .map_err(|err| format!("Could not open file '{}' for reading: {}", file_path, err))?;
    let mut tokio_reader = file_reader.compat();
    let mut contents = String::new();
    tokio_reader.read_to_string(&mut contents).await?;

    println!("CONCLUSION: in the end I was not able to reproduce the bug -- so it may be not a compiler bug afterall...");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn compiler_bug() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = "non-existing-file.txt";
        let file_reader = async_std::fs::File::open(file_path).await
            .map_err(|err| format!("Could not open file '{}' for reading: {}", file_path, err))?;
        let mut tokio_reader = file_reader.compat();
        let mut contents = String::new();
        tokio_reader.read_to_string(&mut contents).await?;
        Ok(())
    }
}