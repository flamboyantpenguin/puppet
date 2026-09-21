use iced::Task;

use crate::gui::app::Message;

pub(crate) fn load_image(url: String) -> Task<Message> {
    let rt = crate::app::runtime::get_runtime();

    let handle = rt.spawn(async move {
        if let Some(path) = url.strip_prefix("file://") {
            tokio::fs::read(path).await.map_err(|e| e.to_string())
        } else {
            let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;

            let bytes = response.bytes().await.map_err(|e| e.to_string())?;

            Ok(bytes.to_vec())
        }
    });

    Task::perform(
        async move { handle.await.map_err(|e| e.to_string())? },
        Message::ImageLoaded,
    )
}
