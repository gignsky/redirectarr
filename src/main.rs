// ============================================================================
// IMPORTS - What we're bringing into scope
// ============================================================================

// From axum crate - the web framework we're using
use axum::{
    Router,       // The main router that maps URLs to handler functions
    routing::get, // Helper function to create a GET route
};

// From tracing_subscriber crate - sets up our logging system
use tracing_subscriber;

// From std library - networking primitives
use std::net::SocketAddr;

// ============================================================================
// MAIN FUNCTION - Entry point of our program
// ============================================================================

// The #[tokio::main] macro transforms our async main into a regular main
// that sets up the Tokio runtime. This is REQUIRED for async/await to work.
//
// What it does behind the scenes:
//   fn main() {
//       tokio::runtime::Runtime::new()
//           .unwrap()
//           .block_on(async { /* our code here */ })
//   }
#[tokio::main]
async fn main() {
    // ========================================================================
    // STEP 1: Initialize logging/tracing
    // ========================================================================

    // This sets up tracing to output to stdout with formatting
    // - Shows timestamps
    // - Shows log levels (INFO, DEBUG, ERROR, etc.)
    // - Shows the target (which module/function logged)
    //
    // tracing_subscriber::fmt() returns a builder
    // .init() consumes the builder and sets it as the global default
    tracing_subscriber::fmt::init();

    // Now we can use tracing macros throughout our code!
    // This is better than println! because:
    //   - Structured data (can filter by level, module, etc.)
    //   - Works well with log aggregation systems
    //   - Can be configured to output JSON for parsing
    tracing::info!("🚀 Redirectarr starting up...");

    // ========================================================================
    // STEP 2: Build the router - define what URLs map to what handlers
    // ========================================================================

    // Router::new() creates an empty router
    // .route(path, method_handler) adds a route
    //   - path: the URL path (e.g., "/", "/health", "/api/users")
    //   - method_handler: what HTTP method + what function to call
    //
    // get(handler_function) means: on HTTP GET request, call handler_function
    //
    // We're using a "closure" (anonymous function) as our handler:
    //   || async { ... }
    //      ^      ^
    //      |      |
    //      |      +-- This is an async block (returns a Future)
    //      +--------- These are the parameters (none in this case)
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // What happens here:
    // - When someone makes a GET request to "http://localhost:8080/"
    // - Axum calls our closure
    // - The closure returns the string "Hello, World!"
    // - Axum automatically converts it to an HTTP response with:
    //     * Status: 200 OK
    //     * Content-Type: text/plain
    //     * Body: "Hello, World!"

    // ========================================================================
    // STEP 3: Define the address and port to bind to
    // ========================================================================

    // SocketAddr is like an IP:Port pair
    // 0.0.0.0 means "listen on all network interfaces"
    //   - localhost (127.0.0.1) would only accept local connections
    //   - 0.0.0.0 accepts connections from anywhere (needed for containers)
    //
    // .parse().unwrap() converts the string to a SocketAddr
    //   - unwrap() crashes if the string is invalid (fine for hardcoded values)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    tracing::info!("🌐 Listening on http://{}", addr);

    // ========================================================================
    // STEP 4: Create and start the server
    // ========================================================================

    // axum::serve() is a convenience function that:
    //   1. Creates a TCP listener on the given address
    //   2. Accepts incoming connections
    //   3. Parses HTTP requests
    //   4. Routes them through our Router
    //   5. Sends HTTP responses back
    //
    // It returns a Server struct
    //
    // tokio::net::TcpListener::bind(addr) creates the listener
    //   - This is async because binding might take time
    //   - .await pauses execution until the listener is ready
    //   - .unwrap() crashes if binding fails (e.g., port already in use)
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("✅ Server ready to accept connections");

    // axum::serve(listener, app) starts serving requests
    //   - listener: where to accept connections from
    //   - app: the Router we built (converted to a Service via into_make_service())
    //
    // .into_make_service() converts our Router into a "MakeService"
    //   - This is a factory that creates a new service for each connection
    //   - Required by the tower/hyper ecosystem
    //
    // .await pauses here FOREVER (or until error/shutdown)
    //   - The server runs in a loop accepting connections
    //   - This is why we need async - we're waiting for network events
    //
    // .unwrap() crashes if the server encounters a fatal error
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    // If we reach here, the server has stopped (error or graceful shutdown)
    tracing::info!("👋 Server shut down");
}

// ============================================================================
// HOW THE PIECES FIT TOGETHER
// ============================================================================
//
// 1. TOKIO (async runtime):
//    - Provides the event loop that drives async/await
//    - Manages OS-level async I/O (TCP sockets, files, timers, etc.)
//    - The #[tokio::main] macro sets this up for us
//    - Everything with .await is scheduled on this runtime
//
// 2. AXUM (web framework):
//    - Built on top of tokio
//    - Handles HTTP protocol details (parsing, serialization)
//    - Provides Router for mapping URLs to handlers
//    - Handles converting Rust types to HTTP responses
//
// 3. TOWER (middleware):
//    - Axum is built on Tower's "Service" trait
//    - Services are composable request/response handlers
//    - We're not using it directly yet, but it's a dependency of axum
//    - Later we'll use it for middleware (logging, timeouts, etc.)
//
// 4. TRACING (logging):
//    - Structured logging framework
//    - Better than println! for production code
//    - Can be configured for different outputs (console, files, JSON, etc.)
//    - tracing_subscriber sets up the "subscriber" that formats/outputs logs
//
// ============================================================================
// WHAT HAPPENS WHEN YOU RUN THIS
// ============================================================================
//
// 1. Program starts, tokio::main sets up async runtime
// 2. Tracing subscriber is initialized
// 3. Router is created with one route: GET / -> "Hello, World!"
// 4. TCP listener binds to 0.0.0.0:8080
// 5. Server starts accepting connections in a loop
// 6. When a request comes in:
//    a. Tokio detects the incoming connection
//    b. Axum parses the HTTP request
//    c. Router matches the path "/" with method GET
//    d. Our closure is called
//    e. Returns "Hello, World!"
//    f. Axum serializes it to HTTP response
//    g. Response sent back to client
// 7. Loop continues until program is killed (Ctrl+C)
//
// ============================================================================
// NEXT STEPS TO TEST
// ============================================================================
//
// In terminal 1:
//   $ cargo run
//
// In terminal 2:
//   $ curl http://localhost:8080
//   Hello, World!
//
//   $ curl http://localhost:8080/nonexistent
//   (Should get 404 Not Found - no route defined for that path)
//
// To stop server:
//   Ctrl+C in terminal 1
//
// ============================================================================
