use std::io::copy;
use std::fs::File;
use tempfile::Builder;
use std::error::Error;

#[tokio::main]
pub async fn csv(url: &String) -> Result<String, Box<dyn Error>> {
    // Downloads a .csv file to a temporary directory (deletes on program exit)
    // On MacOS this this is /var/folders/../../filename.csv
    // Returns string containing directory of file

    // Specify output dir
    let tmp_dir = Builder::new().prefix("tmp-data").tempdir()?;

    // Get file
    let response = reqwest::get(url).await?;

    let mut output_dir = String::new();

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("File to download: {}", fname);
        let fname = tmp_dir.path().join(fname);
        println!("Located at: {:?}", fname);
        output_dir = String::from(fname.to_string_lossy());
        File::create(fname)?
    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(output_dir)
}
