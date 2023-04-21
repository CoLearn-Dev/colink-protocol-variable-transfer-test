use colink::{CoLink, Participant, ProtocolEntry};

struct Initiator;
#[colink::async_trait]
impl ProtocolEntry for Initiator {
    async fn start(
        &self,
        cl: CoLink,
        param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let key = "output";
        let key2 = "output_remote_storage";
        cl.send_variable(key, &param, &participants[1..participants.len()])
            .await?;
        cl.send_variable_with_remote_storage(key2, &param, &participants[1..participants.len()])
            .await?;
        Ok(())
    }
}

struct Receiver;
#[colink::async_trait]
impl ProtocolEntry for Receiver {
    async fn start(
        &self,
        cl: CoLink,
        _param: Vec<u8>,
        participants: Vec<Participant>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let key = "output";
        let key2 = "output_remote_storage";
        let msg = cl.recv_variable(key, &participants[0]).await?;
        cl.create_entry(&format!("tasks:{}:output", cl.get_task_id()?), &msg)
            .await?;
        let msg = cl
            .recv_variable_with_remote_storage(key2, &participants[0])
            .await?;
        cl.create_entry(
            &format!("tasks:{}:output_remote_storage", cl.get_task_id()?),
            &msg,
        )
        .await?;
        Ok(())
    }
}
colink::protocol_start!(
    ("vt_test:initiator", Initiator),
    ("vt_test:receiver", Receiver)
);
