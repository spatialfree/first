use stardust_xr_fusion::{
    client::Client,
    drawable::{self, Text, TextStyle},
    // spatial::Spatial,
    core::values::Transform,
};
use glam::Quat;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

    // Create a text object
    let _txt = Text::create(
        client.get_root(),
        Transform::from_position_rotation_scale(
			[0.0, 0.0, -1.0],
			Quat::from_rotation_y(to_radian(180.0)),
            [0.1, 0.1, 0.1],
		),
        "Hello, stardust!",
        TextStyle::default(),
    ).unwrap();
    


    
    // show off add_numbers
    println!("add_numbers(1, 2) = {}", add_numbers(1, 2));
    
    let _x = 5; // declare and initialize x with a value of 5
    let mut _y = 10; // declare and initialize y with a value of 10, and mark it as mutable so it can be changed later
    
    for i in 0..10 { // loop from 0 to 9 (inclusive)
        print!("{}", i);
    }
    
    let mut j = 0;
    let mut str: String = String::from("Hello, stardust!");
    while j < 10 { // loop while j is less than 10
        str.push_str("!");
        j += 1;
    }
    println!("{}", str);
    
    tokio::select! {
        biased;
        _ = tokio::signal::ctrl_c() => (),
        e = event_loop => e.unwrap().unwrap(),
    }
}

fn add_numbers(x: i32, y: i32) -> i32 { 
    x + y
}

fn to_radian(deg: f32) -> f32 {
    deg * 3.1415 / 180.0
}
