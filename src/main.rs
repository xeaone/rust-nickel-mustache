
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use std::collections::HashMap;


fn main() {

    //Implement A Server
    let mut server = Nickel::new();

    server.utilize(router! {

        //Simple Response
        get "/hw" => |req,res| {
            "Hello World!"
        }

        //Request & Respone
        get "/hello/:name" => |req, res| {
            format!("Hello : {}",req.param("name"))
        }
    });


    //Route Using Mustache Templateing Agent
    server.get("/", middleware! { |req, res|

        let mut data = HashMap::<&str, &str>::new();
        data.insert("title","Index");

        return res.render("views/index.html", &data);
    });

    //Serve A Static Directory
    server.utilize(StaticFilesHandler::new("views/public/"));



    server.listen("0.0.0.0:8080");
}
