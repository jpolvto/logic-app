use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /incidents` goes to `create_incident`
        .route("/incidents", post(create_incident));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_incident(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateIncident` type
    Json(payload): Json<CreateIncident>,
) -> (StatusCode, Json<Incident>) {
    let response = retrieve_data("1").await;

    //get the id for every element in the data array in the response json object and add it to an array
    let mut ids: Vec<u64> = Vec::new();
    match response {
        Ok(body) => {
            let v: serde_json::Value = serde_json::from_str(&body).unwrap();
            let data = v["data"].as_array().unwrap();
            for i in data {
                let id = i["id"].as_u64().unwrap();
                ids.push(id);
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    //Map priority the other way around for the incoming CreateIncident object, if it is 4, map iit as 1, if it is 3, map it as 2, if it is 2, map it as 3, if it is 1, map it as 4
    let priority = match payload.priority {
        4 => 1,
        3 => 2,
        2 => 3,
        1 => 4,
        _ => 0,
    };

    let incident = Incident {
        id: 10,
        incidentname: payload.incidentname,
        priorty: priority,
    };

    //post the incident object to the api

    let result = post_data(&serde_json::to_string(&incident).unwrap()).await;

    // foreach id in the ids array, call put_data with the id
    for id in ids {
        let result = put_data(&serde_json::to_string(&incident).unwrap()).await;
    }
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(incident))
}

// the input to our `create_incident` handler
#[derive(Deserialize)]
struct CreateIncident {
    incidentname: String,
    priority: u64,
}

// the output to our `create_incident` handler
#[derive(Serialize)]
struct Incident {
    id: u64,
    priorty: u64,
    incidentname: String,
}

async fn retrieve_data(input: &str) -> Result<String, Error> {
    let url = format!("https://reqres.in/api/incidents?page={}", input);

    let body = reqwest::get(&url).await?.text().await?;

    Ok(body)
}

async fn post_data(input: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://reqres.in/api/incidents")
        .body(input.to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

// Create a function called put_data that takes a string as input and returns a Result<String, Error> to the https://reqres.in/api/incidents?page={} endpoint, putting only the comments field
async fn put_data(input: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .put("https://reqres.in/api/incidents")
        .body(input.to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}
