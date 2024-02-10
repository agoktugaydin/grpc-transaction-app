use tonic::{transport::Server, Request, Response, Status};

use payments::sol_server::{Solana , SolanaServer};
use payments::{SOLPaymentRequest, SOLPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct SolanaService {}

#[tonic::async_trait]
impl Solana for SolanaService {
    async fn send_payment(
        &self,
        request: Request<SOLPaymentRequest>,
    ) -> Result<Response<SOLPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        let reply = SOLPaymentResponse {
            succesful: true;
            message: format!("Sent {} SOL to {}.", req.amount, req.to).into(),
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
