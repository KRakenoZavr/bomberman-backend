mod bus;
mod game;
mod server;

#[macro_use]
extern crate rocket;

// TODO
// tokio select!
// 1 => listen.accept() => len = 4; break

#[launch]
async fn rocket() -> _ {
    //let p = game::player::Player {};
    //println!("nice");
    //rocket::build().manage(state)
    rocket::build().mount("/", routes![])
}
