use stardust_xr_fusion::client::Client;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let (_client, event_loop) = Client::connect_with_async_loop().await.unwrap();
    
    println!("Hello, stardust!"); 
    
    tokio::select! {
        biased;
        _ = tokio::signal::ctrl_c() => (),
        e = event_loop => e.unwrap().unwrap(),
    }
}

/*  graveyard
    println!("Hello, world!"); 

*/
