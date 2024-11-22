use reqwest::blocking::{Client};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Serialize;

#[derive(Serialize)]
struct ClaimRequest {
    address: String,
}

fn claim_faucet() -> Result<String, reqwest::Error> {
    // Membuat client reqwest
    let client = Client::new();

    // URL API
    let url = "https://faucet.testnet.humanity.org/api/claim";

    // Membuat payload JSON
    let body = ClaimRequest {
        address: "YOUR_ADDRESS".to_string(),
    };

    // Membuat header dengan Content-Type: application/json
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    // Mengirim permintaan POST dengan body JSON
    let response = client
        .post(url)
        .headers(headers)
        .json(&body) // Mengirim JSON sebagai body
        .send();

    // Menangani respons
    match response {
        Ok(resp) => {
            // Jika status response berhasil (kode status 2xx), ambil body respons
            if resp.status().is_success() {
                let response_body = resp.text()?;
                Ok(response_body)
            } else {
                // Jika status response bukan sukses, kembalikan error berdasarkan status respons
                Err(reqwest::Error::from(resp.error_for_status().unwrap_err()))
            }
        }
        Err(e) => {
            // Jika ada error saat mengirim permintaan, kembalikan error tersebut
            Err(e)
        }
    }
}

fn main() {
    match claim_faucet() {
        Ok(response) => println!("Response: {}", response),
        Err(e) => println!("Error: {}", e),
    }
}
