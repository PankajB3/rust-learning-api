

mod models;

use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer};
use mongodb::bson::Document;
use crate::models::User;
use crate::models::Task;
use crate::models::updateBody;
// use futures_util::stream::stream::StreamExt;
use futures::stream::{StreamExt, TryStreamExt};


use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

const DB_NAME: &str = "actix-todo";
const COLL_NAME: &str = "users";
const TASK_COLL_NAME :&str = "tasks";

/// Adds a new user to the "users" collection in the database.
#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Json<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Creates an index on the "username" field to force the values to be unique.
async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

// to-do-api
#[post("/add_task")]
async fn add_task(client:web::Data<Client>, taskData:web::Json<Task>) -> HttpResponse{
    let collection:mongodb::Collection<Task> = client.database(DB_NAME).collection("tasks");
    
    //todo checking if this title exists
    // let allTasks = collection.find(Document::new(),None).await;
    // match allTasks{
    //     Ok(mut result) => {
    //         while let Some(data) =  result.next().await{
    //             if(data.unwrap().title == taskData.title){
    //                 return HttpResponse::InternalServerError().body("err".to_string())
    //             }else{
    //                 collection.insert_one(taskData.into_inner(), None).await;
    //             }
    //         }
    //     },
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()), 
    // }

    let result = collection.insert_one(taskData.into_inner(), None).await;
    // format!("{}", result);
    match result {
        Ok(_) => HttpResponse::Ok().body("Task Added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/task")]
async fn get_all_task(client:web::Data<Client>) -> HttpResponse {
    // let email = path.into_inner();
    let collection:Collection<Task> = client.database(DB_NAME).collection("tasks");
    let result = collection.find(Document::new(), None).await;
    // format!("{}", result);
    let mut resp:Vec<Task> = Vec::new();
    match result {
        Ok(mut result) => { 
            while let Some(data) = result.next().await{
                resp.push(data.unwrap());
            }
            HttpResponse::Ok().json(resp)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[put("/task/{title}")]
async fn update_task(client:web::Data<Client>, path:web::Path<String>, updateBody:web::Json<updateBody>) -> HttpResponse {
    let title = path.into_inner();
    let collection:Collection<Task> = client.database(DB_NAME).collection("tasks");
    let bson_document = match Document::from_reader(title.as_bytes()) {
        Ok(doc) => doc,
        Err(err) => {
            err
            // eprintln!("Failed to parse JSON: {}", err);
        }
    };

    let content_bson = match Document::from_reader(updateBody.content.as_bytes()){
        Ok(content) => content,
        Err(err) => err,
    };
    let result = collection.find_one_and_update(bson_document, content_bson, None).await;
    HttpResponse::Ok().body("")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    // create_username_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            .service(get_user)
            .service(add_task)
            .service(get_all_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// mod models;
// mod config;


// use crate::models::Status;
// use actix_web::{web, HttpServer, Responder, App, HttpResponse};
// use dotenv::dotenv;
// // use mongodb::{options::ClientOptions, Client};
// use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
// // use std::io;

// async fn status() -> impl Responder{
//     // return "Home page"
//     HttpResponse::Ok()
//     .json(Status{status:"Running @ 8080".to_string()})
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()>{

//     //loading env variables into our envirnoment
//     dotenv().ok(); 


//     // // creating connection with mongodb
//     // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
//     // client_options.app_name = Some("actix-todo".to_string());

//     // // creating mongodb client
//     // let client = Client::with_options(client_options).unwrap();

//     // // create mongodb connection pool
//     // let db = client.database("actix-todo");
//     // let pool = web::Data::new(db);

//     let uri = "mongodb://localhost:27017";
//     let client = Client::with_uri_str(uri).await.expect("failed to connect");

 
//     // get instance of configuratiom
//     let config = crate::config::Config::from_env().unwrap();

//     HttpServer::new(move || {
//         App::new()
//         .app_data(web::Data::new(client.clone()))
//         .route("/",web::get().to(status))
//     })
//     .bind(format!("{}:{}",config.server.host, config.server.port))?
//     .run()
//     .await
// }