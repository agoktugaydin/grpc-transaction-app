use payments::solana_client::SolanaClient;
use payments::SolPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SolanaClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        SolPaymentRequest {
            from_addr: "123456".to_owned(),
            to_addr: "654321".to_owned(),
            amount: 22
        }
    );

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}