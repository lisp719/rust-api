use actix_web::{get, guard, post, web, web::Data, App, HttpResponse, HttpServer, Responder};
use async_graphql::{
    http::{GraphiQLSource, MultipartOptions},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::{FilesSchema, MutationRoot, QueryRoot, Storage};

pub mod graphql;

async fn index(schema: web::Data<FilesSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("GraphiQL IDE: http://localhost:8080/graphql/");

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/graphql")
                    .app_data(Data::new(schema.clone()))
                    .service(
                        web::resource("")
                            .guard(guard::Post())
                            .to(index)
                            .app_data(MultipartOptions::default().max_num_files(3)),
                    )
                    .service(web::resource("").guard(guard::Get()).to(gql_playgound)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
