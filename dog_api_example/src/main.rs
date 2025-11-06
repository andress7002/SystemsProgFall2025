use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
    FileError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

fn download_image(url: &str, filename: &str) -> ApiResult{
    match ureq::get(url).call(){
        Ok(response) => {
            
            let mut reader = response.into_reader();

            let mut file = match File::create(filename){
                Ok(f) => f,
                Err(e) => return ApiResult::FileError(format!("Failed to create file: {}", e)),

            };

            if let Err(e) = std ::io::copy(&mut reader, &mut file){
                return ApiResult::FileError(format!("Failed to write file: {}", e));

            }

            ApiResult::Success(DogImage {
                message: filename.to_string(),
                status: "saved".to_string(),
            })
        }
        Err(e) => ApiResult::NetworkError(format!("Failed to download image: {}", e)),
    }





}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);

        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                let filename = format!("dog_image_{}.jpg", i);

                match download_image(&dog_image.message, &filename) {
                    ApiResult::Success(_) => println! ("Image saved as{}\n", filename),
                    ApiResult::FileError(e) => println! ("File Error: {}\n", e),
                    ApiResult::NetworkError(e) => println!("Download Error: {}\n", e),
                    _ => println!("Unexpected error while saving image.\n"),
                }
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
            ApiResult::FileError(e) => println!("File Error: {}\n", e),
        }
        println!();
    }


    Ok(())
}