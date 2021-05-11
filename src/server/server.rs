use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("mx" / String)
        .map(|id| format!("{}", id));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}