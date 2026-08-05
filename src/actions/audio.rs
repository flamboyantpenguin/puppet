use rodio::{Decoder, DeviceSinkBuilder, Player};
use stream_download::source::DecodeError;
use stream_download::storage::temp::TempStorageProvider;
use stream_download::{Settings, StreamDownload};

use crate::app::runtime;

pub async fn play(
    url: String,
    time_ms: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reader = match StreamDownload::new_http(
        url.parse()?,
        TempStorageProvider::new(),
        Settings::default(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return Err(e.decode_error().await)?,
    };

    let rt = runtime::get_runtime();

    rt.spawn_blocking(move || {
        let mut stream = DeviceSinkBuilder::open_default_sink()?;
        stream.log_on_drop(false);
        let player = Player::connect_new(stream.mixer());

        let decoder = Decoder::new(reader).inspect_err(|e| println!("Decoder error: {:?}", e))?;

        player.append(decoder);

        if time_ms == 0 {
            player.sleep_until_end();
        } else {
            std::thread::sleep(std::time::Duration::from_secs(time_ms));
        }

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await??;

    Ok(())
}
