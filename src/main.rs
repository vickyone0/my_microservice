use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use futures::Future;
use std::time::Duration;

fn create_producer() -> FutureProducer {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092")
            .set("client.id", "my-rust-producer")
            .set("acks", "all")
            .set("linger.ms", "5")
            .set("compression.type", "gzip");

            config.create().expect("Producer creation error")
}
#[tokio::main]
async fn main() {
    let producer = create_producer();
    let topic_name = "my_topic";
    
    for i in 0..10 {
        let key = format!("key-{}", i);
        let payload = format!("message-{}", i);
        let record = FutureRecord::to(topic_name)
            .payload(&payload)
            .key(&key);

        let delivery_future = producer.send(record, Duration::from_secs(0));

        delivery_future.await.map(move |delivery_status| {
            println!("Message sent: key={}, payload={}, delivery_status={:?}", key, payload, delivery_status);
        })
        .unwrap();
    }

    println!("All messages sent successfully.");
}