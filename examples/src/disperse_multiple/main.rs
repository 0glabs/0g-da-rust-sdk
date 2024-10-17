use zg_da_rust_sdk::DaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut da_client = DaClient::new("http://0.0.0.0:51001").await?;

    let blob_size = 1024 * 1024 * 32;
    let mut data = vec![0; blob_size];
    for (i, val) in data.iter_mut().enumerate() {
        *val = i as u8;
    }

    println!("uploading blob");
    let blob_header = da_client
        .split_and_disperse_blob_with_finalize(data.clone())
        .await?;
    println!("blob stored: {:?}", blob_header);

    Ok(())
}
