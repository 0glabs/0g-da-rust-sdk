use zg_da_rust_sdk::DaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut da_client = DaClient::new("http://0.0.0.0:51001").await?;
    let data = vec![1, 4];
    println!("uploading blob");
    let blob_header = da_client.disperse_blob_with_finalize(data.clone()).await?;
    println!("blob stored: {:?}", blob_header);

    println!("retrieval and verifying blob");
    let data_retrieved = da_client
        .retrieve_blob(
            blob_header.storage_root,
            blob_header.epoch,
            blob_header.quorum_id,
        )
        .await?
        .data;

    for i in 0..data.len() {
        assert_eq!(data[i], data_retrieved[i]);
    }
    println!("blob verified");

    Ok(())
}
