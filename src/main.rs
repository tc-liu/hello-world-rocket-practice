#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket_instance() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket_instance().launch();
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use crate::rocket_instance;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket_instance()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
