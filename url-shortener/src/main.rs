use warp::Filter;
#[tokio::main]
async fn main() {
    let hi = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, your agent is {}", param, agent));
    let goodbye = warp::path("goodbye")
        .and(warp::path::param())
        .map(|name: String| format!("Good Bye {}", name,));
    let hi = hi.or(goodbye);
    warp::serve(hi).run(([127, 0, 0, 1], 7500)).await
}
