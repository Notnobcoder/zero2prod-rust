<!-- before that all the basic  will be read only will do later-->
<!-- page 14 start -->

project setup
creating a new project
put it in github

<!-- important  -->
<!-- innter development loop -->

Make a change
Compile the application
Run Tests
Run the application

just used 3 packages
cargo install cargo-watch
cargo watch -x check -x test -x run

cargo test
// to run the test cases in the project

// for code coverage -- i dont know what code coverage means
cargo install cargo-tarpaulin

// for auditing the packages
cargo install cargo-audit

// tilll page 20 above

2 . Building an Email Newsletter

// depp dive into the code
HttpServer HttpServer is the backbone supporting our application. It takes
care of things like:
• where should the application be listening for incoming requests? A TCP socket (e.g. 127.0.0.1:8000)?
A Unix domain socket?
• what is the maximum number of concurrent connections that we should allow? How many new
connections per unit of time?
• should we enable transport layer security (TLS)?
• etc.
HttpServer, in other words, handles all transport level concerns.
What happens afterwards? What does HttpServer do when it has established a new connection with
a client of our API and we need to start handling their requests?
That is where App comes into play!
