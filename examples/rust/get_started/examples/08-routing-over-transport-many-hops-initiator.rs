// This node routes a message, to a worker on a different node, over two tcp transport hops.

use ockam::{route, Context, Result, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Initialize the TCP Transport.
    let _tcp = TcpTransport::create(&ctx).await?;

    // Send a message to the "echoer" worker, on a different node, over two tcp hops.
    ctx.send(
        route![
            (TCP, "localhost:3000"), // middle node
            (TCP, "localhost:4000"), // responder node
            "echoer"
        ], // echoer worker on responder node
        "Hello Ockam!".to_string(),
    )
    .await?;

    // Wait to receive a reply and print it.
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await
}
