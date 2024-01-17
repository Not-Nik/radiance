use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any()
        .and(warp::path::full()) // Extract the full path
        .map(|path: warp::path::FullPath| {
            // Access the path using path.as_str()
            println!("{}", path.as_str());
            format!("{}", path.as_str())
        });

    warp::serve(warp::get().and(updates_stable).or(routes))
        .tls()
        // RSA
        .cert_path("certs/cert.pem")
        .key_path("certs/key.pem")
        .run(([0, 0, 0, 0], 443))
        .await;
}
