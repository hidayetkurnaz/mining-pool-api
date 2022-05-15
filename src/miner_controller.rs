use {
    actix_web::web::Json,
    actix_web::HttpResponse,
    crate::miner::*,
    crate::util::*,
    };


//List all Miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    /*
    TODO: Get all MinerDAO objects from DB and convert to Miner objects
    */
    let miners:Vec<Miner> = vec![]; //Empty for now
}