use reqwest::{ Client, header::{ COOKIE } , Error };

#[cfg(test)] use mockito;

#[cfg(not(test))] use std::env;
#[cfg(not(test))] use dotenv;
#[cfg(not(test))] const SESSION : &str = "session";

pub async fn get_input(
    year: i64,
    day: i64
) -> Result<String, Error> {
    #[cfg(not(test))]
    dotenv::dotenv().ok();

    #[cfg(not(test))] let token = env::var(SESSION).unwrap();
    #[cfg(test)] let token : String = "token".to_string();

    #[cfg(not(test))] let base_url = "https://adventofcode.com";
    #[cfg(test)] let base_url = &mockito::server_url();
    
    Ok(Client::new().get(&format!("{}/{}/day/{}/input", base_url, year, day))
        .header(COOKIE, ["session", token.as_str()].join("="))
        .send().await?
        .text().await?)
}