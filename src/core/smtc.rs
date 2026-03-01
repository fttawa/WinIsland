use std::sync::{Arc, Mutex};
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionMediaProperties,
};
use windows::Foundation::TypedEventHandler;
use windows::Storage::Streams::{DataReader, Buffer};

#[derive(Clone, Default, Debug)]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub is_playing: bool,
    pub app_id: String,
    pub thumbnail: Option<Vec<u8>>,
}

pub struct SmtcListener {
    info: Arc<Mutex<MediaInfo>>,
}

impl SmtcListener {
    pub fn new() -> Self {
        let listener = Self {
            info: Arc::new(Mutex::new(MediaInfo::default())),
        };
        listener.init();
        listener
    }

    pub fn get_info(&self) -> MediaInfo {
        self.info.lock().unwrap().clone()
    }

    fn init(&self) {
        let info_clone = self.info.clone();
        std::thread::spawn(move || {
            let manager_op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync();
            if let Ok(manager_async) = manager_op {
                if let Ok(manager) = manager_async.get() {
                    let manager: GlobalSystemMediaTransportControlsSessionManager = manager;
                    let _ = Self::update_all(&manager, &info_clone);

                    let info_for_handler = info_clone.clone();
                    let handler = TypedEventHandler::new(move |m: &Option<GlobalSystemMediaTransportControlsSessionManager>, _| {
                        if let Some(mgr) = m {
                            let _ = Self::update_all(mgr, &info_for_handler);
                        }
                        Ok(())
                    });
                    let _ = manager.SessionsChanged(&handler);

                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        let _ = Self::update_all(&manager, &info_clone);
                    }
                }
            }
        });
    }

    fn update_all(manager: &GlobalSystemMediaTransportControlsSessionManager, info_arc: &Arc<Mutex<MediaInfo>>) -> windows::core::Result<()> {
        if let Ok(session) = manager.GetCurrentSession() {
            Self::update_session(&session, info_arc)?;
        } else {
            let mut info = info_arc.lock().unwrap();
            *info = MediaInfo::default();
        }
        Ok(())
    }

    fn update_session(session: &GlobalSystemMediaTransportControlsSession, info_arc: &Arc<Mutex<MediaInfo>>) -> windows::core::Result<()> {
        let app_id = session.SourceAppUserModelId()?.to_string();
        let props_async = session.TryGetMediaPropertiesAsync()?;
        let props: GlobalSystemMediaTransportControlsSessionMediaProperties = props_async.get()?;
        
        let pb_info = session.GetPlaybackInfo()?;
        let is_playing = pb_info.PlaybackStatus()? == windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;

        let thumb_bytes = if let Ok(thumb_ref) = props.Thumbnail() {
            if let Ok(stream_async) = thumb_ref.OpenReadAsync() {
                if let Ok(stream) = stream_async.get() {
                    let size = stream.Size()? as u32;
                    let buffer = Buffer::Create(size)?;
                    if let Ok(read_async) = stream.ReadAsync(&buffer, size, windows::Storage::Streams::InputStreamOptions::None) {
                        if let Ok(res_buffer) = read_async.get() {
                            let reader = DataReader::FromBuffer(&res_buffer)?;
                            let mut bytes = vec![0u8; size as usize];
                            reader.ReadBytes(&mut bytes)?;
                            Some(bytes)
                        } else { None }
                    } else { None }
                } else { None }
            } else { None }
        } else { None };

        let mut info = info_arc.lock().unwrap();
        info.title = props.Title()?.to_string();
        info.artist = props.Artist()?.to_string();
        info.album = props.AlbumTitle()?.to_string();
        info.is_playing = is_playing;
        info.app_id = app_id;
        info.thumbnail = thumb_bytes;
        
        Ok(())
    }
}
