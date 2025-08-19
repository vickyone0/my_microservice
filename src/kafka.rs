use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::KafkaError;
use futures::executor::block_on;


async fn produce_message() -> Result<(), KafkaError> {
    let brokers = "localhost:9092";
    let topic_name = "my_topic";
    // Create a producer instance.
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .set("acks", "all") // Ensure all brokers acknowledge the message
        .create()?;
    // Prepare the message to be sent.
    let payload = "Hello, Kafka!";
    let key = "my_key";
    let record = FutureRecord::to(topic_name)
        .payload(payload)
        .key(key);
    // Send the message.
    let delivery_status = producer.send(record, std::time::Duration::from_secs(0)).await;
    // Check the delivery status.
    match delivery_status {
        Ok(_) => {
            println!("Message delivered successfully");
            Ok(())
        },
        Err((e, _)) => {
            eprintln!("Failed to deliver message: {}", e);
            Err(e)
        }
    }
}


// use rdkafka::ClientConfig;
// use rdkafka::consumer::{StreamConsumer, Consumer};
// use rdkafka::message::Message;
// use rdkafka::error::KafkaError;
// use futures::stream::StreamExt;
// async fn consume_messages() -> Result<(), KafkaError> {
//     let brokers = "localhost:9092";
//     let group_id = "my_group";
//     let topic_name = "my_topic";
//     // Create a consumer instance.
//     let consumer: StreamConsumer = ClientConfig::new()
//         .set("bootstrap.servers", brokers)
//         .set("group.id", group_id)
//         .set("enable.auto.commit", "false") // Disable auto-commit
//         .set("session.timeout.ms", "6000")
//         .set("enable.partition.eof", "false")
//         .create()?;
//     // Subscribe to the topic.
//     consumer.subscribe(&[topic_name])?;
//     // Create the stream.
//     let mut message_stream = consumer.stream();
//     // Process messages.
//     while let Some(Ok(message)) = message_stream.next().await {
//         println!("Received message: {:?}", message);
//         // Manually commit the message offset.
//         consumer.commit_message(&message, rdkafka::consumer::CommitMode::Sync)?;
//     }
//     Ok(())
// }