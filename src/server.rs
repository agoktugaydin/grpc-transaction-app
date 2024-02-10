use tonic::{transport::Server, Request, Response, Status};

use crate::payments::solana_server::{Solana, SolanaServer};
use crate::payments::{SolPaymentRequest, SolPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct SolanaService {}

#[tonic::async_trait]

impl Solana for SolanaService {
    async fn send_payment(
        &self,
        request: Request<SolPaymentRequest>,
    ) -> Result<Response<SolPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        let reply = SolPaymentResponse {
            successful: true,
            message: format!("Sent {} SOL to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let sol_service = SolanaService::default();

    Server::builder()
        .add_service(SolanaServer::new(sol_service))
        .serve(addr)
        .await?;
    Ok(())
}
