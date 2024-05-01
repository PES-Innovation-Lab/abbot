use prometheus::{ proto::MetricFamily, IntCounter, Registry };
use lazy_static::lazy_static;

lazy_static!{
    pub static ref REGISTRY: Registry = Registry::new();
    
    pub static ref TOTAL_INCOMING_MESSAGES_COUNTER: IntCounter = IntCounter::new("total_incoming_message_count", "count of total incoming messages")
    .expect("should be able to create counter");

    pub static ref TOTAL_HUMAN_MESSAGES_COUNTER: IntCounter = IntCounter::new("total_human_message_count", "count of total incoming human messages")
    .expect("should be able to create counter");

    pub static ref HUMAN_BOT_REQUEST_MSG_COUNTER: IntCounter = IntCounter::new("human_bot_request_msg_counter", "requests to the bot")
    .expect("should be able to create counter");

    pub static ref BOT_SENT_MESSAGES_COUNTER: IntCounter = IntCounter::new("bot_sent_msg_counter", "count of messages sent by bot")
    .expect("should be able to create counter");
}

pub fn register_metrics() {
    REGISTRY.register(Box::new(TOTAL_INCOMING_MESSAGES_COUNTER.clone())).expect("should be able to register counter");
    REGISTRY.register(Box::new(TOTAL_HUMAN_MESSAGES_COUNTER.clone())).expect("should be able to register counter");
    REGISTRY.register(Box::new(HUMAN_BOT_REQUEST_MSG_COUNTER.clone())).expect("should be able to register counter");
    REGISTRY.register(Box::new(BOT_SENT_MESSAGES_COUNTER.clone())).expect("should be able to register counter");
}

pub fn metrics_handler() -> String {
    let mut gathered_metrics_result = String::new();
    gathered_metrics_result.push_str(metrics_gatherer(&REGISTRY.gather()).as_str());
    gathered_metrics_result.push_str(metrics_gatherer(&prometheus::gather()).as_str());
    gathered_metrics_result
}

fn metrics_gatherer(src: &Vec<MetricFamily>) -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();

    if let Err(e) = encoder.encode(src, &mut buffer) {
        eprintln!("Could not encode metrics {}", e);
    }

    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to from_utf8 the gathered metrics {}", e);
            String::default()
        }
    };
    res
}