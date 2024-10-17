pub mod disperser {
    tonic::include_proto!("disperser");
}

use itertools::Itertools;
use tokio::time::{sleep, Duration, Instant};
use tonic::{
    codegen::StdError,
    transport::{Channel, Endpoint},
};

use disperser::{
    disperser_client::DisperserClient, BlobHeader, BlobStatus, BlobStatusReply, BlobStatusRequest,
    DisperseBlobReply, DisperseBlobRequest, RetrieveBlobReply, RetrieveBlobRequest,
};

pub const MAX_BLOB_SIZE: usize = 1024 * 1024 * 31 - 4;
pub const WAIT_BLOB_TIMEOUT_IN_SECS: u64 = 300;

pub struct DaClient {
    client: DisperserClient<Channel>,
    wait_blob_timeout_in_secs: Option<u64>,
}

impl DaClient {
    pub async fn new<D>(dst: D) -> Result<Self, Box<dyn std::error::Error>>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let rpc_endpoint = Endpoint::new(dst)?;
        let channel = rpc_endpoint.connect_lazy();

        let client = DisperserClient::new(channel);

        Ok(Self {
            client,
            wait_blob_timeout_in_secs: None,
        })
    }

    pub fn with_wait_blob_timeout_in_secs(mut self, timeout_in_secs: u64) -> Self {
        self.wait_blob_timeout_in_secs = Some(timeout_in_secs);
        self
    }

    pub async fn disperse_blob(
        &mut self,
        data: Vec<u8>,
    ) -> Result<DisperseBlobReply, Box<dyn std::error::Error>> {
        if data.len() > MAX_BLOB_SIZE {
            return Err(format!(
                "maximum blob size {} exceeded, current blob size {}",
                MAX_BLOB_SIZE,
                data.len()
            )
            .into());
        }

        let request = DisperseBlobRequest { data };
        let response = self.client.disperse_blob(request).await?;
        Ok(response.into_inner())
    }

    pub async fn disperse_blob_with_finalize(
        &mut self,
        data: Vec<u8>,
    ) -> Result<BlobHeader, Box<dyn std::error::Error>> {
        let request_id = self.disperse_blob(data).await?.request_id;
        self.wait_blob_finalized(request_id).await
    }

    pub async fn split_and_disperse_blob(
        &mut self,
        data: Vec<u8>,
    ) -> Result<Vec<DisperseBlobReply>, Box<dyn std::error::Error>> {
        let mut tasks = vec![];
        for chunk in data.into_iter().chunks(MAX_BLOB_SIZE).into_iter() {
            tasks.push(self.disperse_blob(chunk.collect()).await?);
        }

        Ok(tasks)
    }

    pub async fn split_and_disperse_blob_with_finalize(
        &mut self,
        data: Vec<u8>,
    ) -> Result<Vec<BlobHeader>, Box<dyn std::error::Error>> {
        let tasks = self.split_and_disperse_blob(data).await?;
        let mut res = vec![];
        for task in tasks {
            res.push(self.wait_blob_finalized(task.request_id).await?);
        }

        Ok(res)
    }

    pub async fn get_blob_status(
        &mut self,
        request_id: Vec<u8>,
    ) -> Result<BlobStatusReply, Box<dyn std::error::Error>> {
        let request = BlobStatusRequest { request_id };
        let response = self.client.get_blob_status(request).await?;
        Ok(response.into_inner())
    }

    pub async fn retrieve_blob(
        &mut self,
        storage_root: Vec<u8>,
        epoch: u64,
        quorum_id: u64,
    ) -> Result<RetrieveBlobReply, Box<dyn std::error::Error>> {
        let request = RetrieveBlobRequest {
            storage_root,
            epoch,
            quorum_id,
        };
        let response = self.client.retrieve_blob(request).await?;
        Ok(response.into_inner())
    }

    pub async fn wait_blob_finalized(
        &mut self,
        request_id: Vec<u8>,
    ) -> Result<BlobHeader, Box<dyn std::error::Error>> {
        let instant = Instant::now();

        loop {
            let blob_status_reply = self.get_blob_status(request_id.clone()).await?;
            match BlobStatus::try_from(blob_status_reply.status)? {
                BlobStatus::Finalized => {
                    break Ok(blob_status_reply
                        .info
                        .ok_or("blob info is none")?
                        .blob_header
                        .ok_or("blob header is none")?);
                }
                BlobStatus::Failed => {
                    break Err("failed to retrieve blob status".into());
                }
                _ => {
                    if instant.elapsed()
                        > Duration::from_secs(if let Some(v) = self.wait_blob_timeout_in_secs {
                            v
                        } else {
                            WAIT_BLOB_TIMEOUT_IN_SECS
                        })
                    {
                        break Err("blob status retrieval timeout".into());
                    }

                    sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            }
        }
    }
}
