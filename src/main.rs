use rdkafka::{
    ClientConfig,
    consumer::{
        Consumer,
        StreamConsumer,
    },
    error::KafkaResult,
    Message,
    TopicPartitionList,
};

use std::time::Duration;

async fn consume_messages(group_id: &str, topics: &[&str]) -> KafkaResult<()> {

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("section.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()?;

    consumer.subscribe(topics)?;

    loop{
        match consumer.recv().await {
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    Some(Ok(s)) => s,
                    Some(Err(_)) => {
                        println!("Error: message payload is not a string");
                        continue;
                    }
                    None =>"",
                };
                println!("Received message: topic={}, partition={}, offset={}, key={:?}, payload={:?}",
                         m.topic(), m.partition(), m.offset(), m.key(), payload);
            },
            Err(e) => {
                println!("Error receiving message: {}", e);
                
            }
        }
    }
    Ok(())
} 


#[tokio::main]
async fn main() {
    let group_id = "my-rust-consumer-group";
    let topics = ["my_topic"];
    
    if let Err(e) = consume_messages(group_id, &topics).await {
        eprintln!("Error consuming messages: {}", e);
    }
}