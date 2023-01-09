use std::{env, fs, path::Path};

fn main() -> Result<(), anyhow::Error> {
    let iceberg_catalog = env::var("ICEBERG_CATALOG")?;
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID")?;
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY")?;
    println!(
        "{}, {}, {}",
        iceberg_catalog, aws_access_key_id, aws_secret_access_key
    );
    let dirs = fs::read_dir(Path::new("/")).map_err(anyhow::Error::msg)?;
    for dir in dirs {
        match dir {
            Ok(file) => {
                println!("{:?}", file);
            }
            Err(err) => println!("{}", err),
        }
    }
    Ok(())
}
