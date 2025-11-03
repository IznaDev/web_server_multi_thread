In this project I build a multi-thread HTTP server pretty basic but with lot of concepts. 

I build it manually without using any framework like Actix-web, Axum or Rocket, or even Tokio. 

It is not an asynchronous server :

    The I/O operations block the threads that wait the tasks. In asynchronous context with a runtime like tokio, the threads don't block the execution when they perform I/O operations. 

I only use elements provided by the standard Rust library (std) like Tcp, mpsc, thread, Arc, Mutex. 

The purpose of that project is to understand some mechanisms of the concurrency (be carreful I said concurrency not asynchonous) programation especially the channels enabling communication and data sharing between threads. Channels are also used extensively in Golang. 

The principle of channels is perfectly express in this slogan from the Go langage documentation : 

            "Do not communicate by sharing memory; instead share memory by communicating"

The channel is a major tool Rust to handle message-sending concurrency.

There are other ways and tools to handle a huge amount of data in asynchronous ways. THey will be a subject of other projects.

This web server is made up of :

    - a Socket connexion with clients
    - a Thread Pool creating channel and Workers to handle async tasks execution
    - Workers creating thread, wait the tasks, then execute them
    - an execute function that send the the tasks to the Workers
    - a handle_connection function to handle clients requests

Cargo run ! open a browser or several and connect to http://127.0.0.1:7878 and observe what is diplayed.