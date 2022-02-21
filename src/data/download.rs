use std::io::copy;
use std::fs::{File, DirBuilder};
use std::error::Error;

#[tokio::main]
pub async fn csv(url: &String, fname: &String) -> Result<(), Box<dyn Error>> {
    // Downloads a .csv file to a directory from a url

    // Specify output dir
    // recursive(true) means the directory will be overwritten 
    let path = "data/"; // Ouput folder
    let dir = DirBuilder::new()
        .recursive(true)
        .create(path)?;

    // Create the file to write into
    let mut file = {
        let name = format!("{}.csv", fname);
        let output_dir = format!("{}{}", path.to_string(), name);

        println!("New file located at: {:?}", output_dir);

        File::create(&output_dir)?
    };

    // Get file from url
    let content = reqwest::get(url)
        .await?
        .text()
        .await?;

    // Insert the response contents into the file
    copy(&mut content.as_bytes(), &mut file)?;

    Ok(())
}
