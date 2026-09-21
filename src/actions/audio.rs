use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, DeviceSinkBuilder, Player};
use stream_download::source::DecodeError;
use stream_download::storage::temp::TempStorageProvider;
use stream_download::{Settings, StreamDownload};

use crate::app::runtime;

pub async fn play(
    url: String,
    time_ms: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rt = runtime::get_runtime();

    let (stream, player) = if let Some(path) = url.strip_prefix("file://") {
        //let decoder = Decoder::new(file).inspect_err(|e| println!("Decoder error: {:?}", e))?;

        let f = File::open(path)?;
        rt.spawn_blocking(move || {
            let file = BufReader::new(f);

            let mut stream = DeviceSinkBuilder::open_default_sink()?;
            stream.log_on_drop(false);

            let player = rodio::play(stream.mixer(), file)?;

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>((stream, player))
        })
        .await??
    } else {
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

        rt.spawn_blocking(move || {
            let mut stream = DeviceSinkBuilder::open_default_sink()?;
            stream.log_on_drop(false);
            let decoder =
                Decoder::new(reader).inspect_err(|e| println!("Decoder error: {:?}", e))?;

            let player = Player::connect_new(stream.mixer());
            player.append(decoder);

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>((stream, player))
        })
        .await??
    };

    if time_ms == 0 {
        player.sleep_until_end();
    } else {
        std::thread::sleep(std::time::Duration::from_millis(time_ms));
    }

    drop(player);
    drop(stream);

    Ok(())
}
