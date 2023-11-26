use spin_sdk::http::{IntoResponse, Request, Method};
use spin_sdk::http_component;

use serde::Serialize;
use spin_sdk::sqlite::Connection;

// Helper for returning the query results as JSON
#[derive(Serialize)]
struct Statistics {
    components: u32,
}

async fn h(increase: bool) -> Option<Vec<u8>> {
    let connection = Connection::open_default().unwrap();

    if increase {
        connection
            .execute(
                "UPDATE statistics SET components = components + 1",
                &[],
            )
            .expect("msg");
    }
    let rowset = connection
        .execute(
            "SELECT * FROM statistics;",
            &[],
        )
        .expect("msg");    
    let todos: Vec<Statistics>  = rowset
        .rows()
        .map(|row| Statistics {
            components: row.get::<u32>("components").unwrap().to_owned(),
        })
        .collect();

    Some(serde_json::to_vec(&todos[0]).unwrap())
}

/// A simple Spin HTTP component.
#[http_component]
async fn handle_statistic(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let result = match req.method() {
        Method::Get => h(false).await,
        Method::Post => h(true).await,
        _ => h(false).await,
    };
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(result)?)
}
