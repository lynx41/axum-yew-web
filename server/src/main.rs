use dotenvy_macro::dotenv;

use server::run;


#[tokio::main]
async fn main() {
    let database_url = dotenv!("DATABASE_URL");
    
    run(database_url).await;
}
