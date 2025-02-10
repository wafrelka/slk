use anyhow::Context as _;

#[derive(Debug, serde::Serialize)]
pub struct BasicMessage {
    pub color: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct Payload {
    attachments: Vec<AttachmentPayload>,
}

#[derive(Debug, serde::Serialize)]
struct AttachmentPayload {
    color: Option<String>,
    title: Option<String>,
    title_link: Option<String>,
    text: Option<String>,
    mrkdwn_in: Vec<String>,
    ts: Option<u64>,
}

impl From<BasicMessage> for Payload {
    fn from(value: BasicMessage) -> Self {
        let BasicMessage { color, title, title_link, text } = value;
        Payload {
            attachments: vec![AttachmentPayload {
                color,
                title,
                title_link,
                text,
                mrkdwn_in: vec!["text".into()],
                ts: None,
            }],
        }
    }
}

impl Payload {
    fn with_timestamp(mut self, ts: u64) -> Self {
        for a in &mut self.attachments {
            a.ts = Some(ts);
        }
        self
    }
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn post_message(webhook: &str, msg: BasicMessage) -> anyhow::Result<()> {
    let payload: Payload = msg.into();
    let payload = payload.with_timestamp(now());
    let payload_json = serde_json::to_string(&payload).expect("must be serializable");
    let client = reqwest::blocking::ClientBuilder::new()
        .build()
        .context("could not initiate http client")?;
    client.post(webhook).body(payload_json).send().context("could not send request to slack")?;
    Ok(())
}
