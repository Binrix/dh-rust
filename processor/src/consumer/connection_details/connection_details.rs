/// Contains the connection details for the connection to the message broker.
pub struct ConnectionDetails {
    pub stream_id: u32,
    pub topic_id: u32,
    pub partition_id: u32
}

impl Default for ConnectionDetails {
    fn default() -> Self {
        Self {
            partition_id: 1,
            topic_id: 1,
            stream_id: 1
        }
    }
}