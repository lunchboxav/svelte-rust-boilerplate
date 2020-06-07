#![deny(warnings)]

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let server_port = 3030;

    let cors = warp::cors()
        .allow_any_origin();

    // The following code is taken from Warp routing example, with some example removed

    // We'll start simple, and gradually show how you combine these powers
    // into super powers!

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    // Fine, but how do I handle parameters in paths?
    //
    // GET /sum/:u32/:u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // Any type that implements FromStr can be used, and in any order:
    //
    // GET /:u16/times/:u16
    let times =
        warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b));

    // Oh shoot, those math routes should be mounted at a different path,
    // is that possible? Yep.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math");
    let _sum = math.and(sum);
    let _times = math.and(times);

    let math = warp::path("math").and(sum.or(times));

    // We can use the end() filter to match a shorter path
    let help = warp::path("math")
        // Careful! Omitting the following line would make this filter match
        // requests to /math/sum/:u32/:u32 and /math/:u16/times/:u16
        .and(warp::path::end())
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16");
    let math = help.or(math);

    // Let's let people know that the `sum` and `times` routes are under `math`.
    let sum = sum.map(|output| format!("(This route has moved to /math/sum/:u16/:u16) {}", output));
    let times =
        times.map(|output| format!("(This route has moved to /math/:u16/times/:u16) {}", output));

    let static_file = warp::path("static").and(warp::fs::dir("../client"));

    let routes = warp::get().and(hi.or(hello_from_warp).or(math).or(sum).or(times).or(static_file)).with(cors);

    println!("Server is running in port {}", server_port);

    warp::serve(routes).run(([127, 0, 0, 1], server_port)).await;
}
