
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, StaticFilesHandler};

use std::collections::HashMap;


fn main() {

    let mut server = Nickel::new();

    // Simple Responses
    let router = router! {
        get "/root" => |_req, _res| {
            "Root"
        }
    };

    // Route Using Rustache Template
    server.get("/", middleware! { |_req, _res|

        let mut data = HashMap::new();
        data.insert("title","Index");

        return _res.render("views/index.html", &data);
    });


    server.utilize(router);
    server.utilize(StaticFilesHandler::new("views/public/"));
    server.listen("127.0.0.1:8080");
}
