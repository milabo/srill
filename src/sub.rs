pub fn subscribe<'a>(
    redis_url: &'a str,
    channel: &'a str,
    callback: impl Fn(&str) + Send + 'static + Clone,
) -> anyhow::Result<()> {
    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_connection()?;
    let mut pubsub = conn.as_pubsub();
    pubsub.subscribe(channel)?;

    loop {
        let msg: String = pubsub.get_message()?.get_payload()?;
        // 非同期スレッドで callback を呼び出す
        let callback = callback.clone();
        tokio::spawn(async move {
            callback(&msg);
        });
    }
}
