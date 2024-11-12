# 0g-da-rust-sdk

zg-da-rust-sdk provides a simple and efficient way to integrate with 0g DA client, facilitating the dispersal and retrieval of large binary data (blobs). The client supports functionalities such as dispersing data into smaller chunks, retrieving the status of dispersed data, and ensuring that blobs are finalized before being retrieved.

# Features

* **Disperse a Blob**: Send binary data to be dispersed.
* **Split and Disperse Blob**: Split a large blob into smaller chunks and disperse them.
* **Disperse with Finalization**: Disperse a blob and wait until it is finalized before retrieving.
* **Retrieve Blob**: Retrieve a blob using the storage root, epoch, and quorum ID.
* **Blob Status**: Get the status of a dispersed blob.


# APIs

## new
```rust
pub async fn new<D>(dst: D) -> Result<Self, Box<dyn std::error::Error>>
where
    D: std::convert::TryInto<tonic::transport::Endpoint>,
    D::Error: Into<StdError>,
```
* Description:
  
  Creates a new DaClient instance, which connects to the specified dst (the endpoint of the Disperser service).

* Parameters:

  * **dst**: The destination endpoint URL or address for the gRPC connection.
  
* Returns:
  
    A Result containing the newly created DaClient on success, or an error if the connection fails.

## disperse_blob
```rust
pub async fn disperse_blob(
    &mut self,
    data: Vec<u8>,
) -> Result<DisperseBlobReply, Box<dyn std::error::Error>>
```

* Description:
  
    Disperses a blob of binary data. The size of the data must not exceed MAX_BLOB_SIZE. Returns a DisperseBlobReply containing details about the dispersed blob.

* Parameters:

    * **data**: A vector of bytes representing the binary data to be dispersed.

* Returns:

    A Result containing the DisperseBlobReply on success or an error if the blob size exceeds the maximum allowed size or any other issue occurs.

## disperse_blob_with_finalize

```rust
pub async fn disperse_blob_with_finalize(
    &mut self,
    data: Vec<u8>,
) -> Result<BlobHeader, Box<dyn std::error::Error>>
```
* Description:

    Disperses a blob and waits for the blob to be finalized. Returns a BlobHeader once the blob is finalized.

* Parameters:

    * **data**: A vector of bytes representing the binary data to be dispersed.

* Returns:

    A Result containing the BlobHeader upon successful finalization, or an error if the process fails.

## split_and_disperse_blob

```rust
pub async fn split_and_disperse_blob(
    &mut self,
    data: Vec<u8>,
) -> Result<Vec<DisperseBlobReply>, Box<dyn std::error::Error>>
```
* Description:
    
    Splits a large blob into smaller chunks (based on MAX_BLOB_SIZE) and disperses each chunk separately. Returns a list of DisperseBlobReply responses for each chunk.

* Parameters:

    * **data**: A vector of bytes representing the binary data to be split and dispersed.

* Returns:

    A Result containing a vector of DisperseBlobReply responses, or an error if any chunk fails to disperse.

## split_and_disperse_blob_with_finalize

```rust
pub async fn split_and_disperse_blob_with_finalize(
    &mut self,
    data: Vec<u8>,
) -> Result<Vec<BlobHeader>, Box<dyn std::error::Error>>
```

* Description:

    Splits a large blob into smaller chunks, disperses each chunk, and waits for all chunks to be finalized. Returns a list of BlobHeader instances once all chunks are finalized.

* Parameters:

    * **data**: A vector of bytes representing the binary data to be split and dispersed.

* Returns:

    A Result containing a vector of BlobHeader instances on successful finalization, or an error if the process fails.

## get_blob_status

```rust
pub async fn get_blob_status(
    &mut self,
    request_id: Vec<u8>,
) -> Result<BlobStatusReply, Box<dyn std::error::Error>>
```

* Description:
    
    Retrieves the status of a dispersed blob by its request_id.

* Parameters:

    * **request_id**: A vector of bytes representing the unique request ID for the dispersed blob.

* Returns:

    A Result containing a BlobStatusReply on success, or an error if the status cannot be retrieved.

## retrieve_blob

```rust
pub async fn retrieve_blob(
    &mut self,
    storage_root: Vec<u8>,
    epoch: u64,
    quorum_id: u64,
) -> Result<RetrieveBlobReply, Box<dyn std::error::Error>>
```

* Description:
    Retrieves a blob from the storage system using the specified storage_root, epoch, and quorum_id.

* Parameters:

    * **storage_root**: A vector of bytes representing the storage root of the blob.
    * **epoch**: The epoch value associated with the blob.
    * **quorum_id**: The quorum ID associated with the blob.

* Returns:

    A Result containing a RetrieveBlobReply on success, or an error if the retrieval fails.