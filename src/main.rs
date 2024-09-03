use rocket::State;
use sandkasten_client::{
    schemas::programs::{BuildRequest, BuildRunRequest, MainFile, RunRequest},
    SandkastenClient,
};

use std::time;
#[macro_use]
extern crate rocket;

struct ClientState {
    client: SandkastenClient,
}

#[post("/submit_code?<language>", data = "<input>")]
async fn submit_code(state: &State<ClientState>, language: &str, input: &str) -> String {
    let x = state
        .client
        .build(&BuildRequest {
            environment: language.into(),
            main_file: MainFile {
                name: None,
                content: input.into(),
            },
            ..Default::default()
        })
        .await;

    match x {
        Ok(x) => return x.program_id.to_string(),
        Err(x) => return format!("{:?}", x),
    }
}

#[get("/code_result?<id>")]
async fn code_result(state: &State<ClientState>, id: &str) -> String {
    let x = state
        .client
        .run(
            id,
            &RunRequest {
                ..Default::default()
            },
        )
        .await;
    match x {
        Ok(x) => return x.stdout,
        Err(x) => return format!("{:?}", x),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![submit_code, code_result])
        .manage(ClientState {
            client: SandkastenClient::new("https://sandkasten.bootstrap.academy/".parse().unwrap()),
        })
}
