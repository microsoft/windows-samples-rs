use std::sync::mpsc;
use windows::{
    core::HSTRING,
    Foundation::{AsyncOperationWithProgressCompletedHandler, Uri},
    Web::Syndication::SyndicationClient,
};

fn main() -> windows::core::Result<()> {
    let uri = Uri::CreateUri(HSTRING::from("https://kennykerr.ca/feed"))?;
    let client = SyndicationClient::new()?;

    let (sender, receiver) = mpsc::channel();
    client.RetrieveFeedAsync(uri)?.SetCompleted(
        AsyncOperationWithProgressCompletedHandler::new(move |op, _status| {
            if let Some(op) = op {
                sender
                    .send(op.GetResults()?)
                    .expect("send over mpsc channel");
            }
            Ok(())
        }),
    )?;

    let feed = receiver.recv().unwrap();
    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
    }

    Ok(())
}
