use reqwasm::http::Request;
use reqwasm::Error;
use serde::de::DeserializeOwned;


pub async fn request_post<T: DeserializeOwned >(endpoint:&String, body: &String) -> Result<T, Error>{
    let res:Result<T, Error> = Request::post(endpoint)
    .header("Content-Type", "application/json")
    .body(body)
    .send()
    .await
    .unwrap()
    .json()
    .await;
                    
    res
}