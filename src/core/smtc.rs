use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSession,
};
use windows::Foundation::TypedEventHandler;
#[derive(Clone, Default, Debug)]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub is_playing: bool,
    pub thumbnail: Option<Arc<Vec<u8>>>,
    pub spectrum: [f32; 6],
}
pub struct SmtcListener {
    info: Arc<Mutex<MediaInfo>>,
    active: Arc<AtomicBool>,
}
impl SmtcListener {
    pub fn new() -> Self {
        let listener = Self {
            info: Arc::new(Mutex::new(MediaInfo::default())),
            active: Arc::new(AtomicBool::new(true)),
        };
        listener.init();
        listener
    }
    pub fn get_info(&self) -> MediaInfo {
        self.info.lock().unwrap().clone()
    }
    fn init(&self) {
        let info_clone = self.info.clone();
        let active_clone = self.active.clone();
        std::thread::spawn(move || {
            let manager = match GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
                Ok(op) => match op.get() {
                    Ok(m) => m,
                    Err(_) => return,
                },
                Err(_) => return,
            };
            let update_info = |mgr: &GlobalSystemMediaTransportControlsSessionManager, arc: &Arc<Mutex<MediaInfo>>| {
                if let Ok(session) = mgr.GetCurrentSession() {
                    let _ = Self::fetch_properties(&session, arc);
                } else {
                    if let Ok(mut info) = arc.lock() {
                        *info = MediaInfo::default();
                    }
                }
            };
            update_info(&manager, &info_clone);
            let info_for_handler = info_clone.clone();
            let handler = TypedEventHandler::new(move |m: &Option<GlobalSystemMediaTransportControlsSessionManager>, _| {
                if let Some(mgr) = m {
                    let _ = update_info(mgr, &info_for_handler);
                }
                Ok(())
            });
            let _ = manager.SessionsChanged(&handler);
            while active_clone.load(Ordering::Relaxed) {
                update_info(&manager, &info_clone);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });
    }
    fn fetch_properties(session: &GlobalSystemMediaTransportControlsSession, info_arc: &Arc<Mutex<MediaInfo>>) -> windows::core::Result<()> {
        let props = session.TryGetMediaPropertiesAsync()?.get()?;
        let pb_info = session.GetPlaybackInfo()?;
        let is_playing = pb_info.PlaybackStatus()? == windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;
        let mut thumb_data = None;
        if let Ok(thumb_ref) = props.Thumbnail() {
            if let Ok(stream) = thumb_ref.OpenReadAsync()?.get() {
                let size = stream.Size()? as u32;
                let buffer = windows::Storage::Streams::Buffer::Create(size)?;
                let res_buffer = stream.ReadAsync(&buffer, size, windows::Storage::Streams::InputStreamOptions::None)?.get()?;
                let reader = windows::Storage::Streams::DataReader::FromBuffer(&res_buffer)?;
                let mut bytes = vec![0u8; size as usize];
                reader.ReadBytes(&mut bytes)?;
                thumb_data = Some(Arc::new(bytes));
            }
        }
        if let Ok(mut info) = info_arc.lock() {
            info.title = props.Title()?.to_string();
            info.artist = props.Artist()?.to_string();
            info.album = props.AlbumTitle()?.to_string();
            info.is_playing = is_playing;
            info.thumbnail = thumb_data;
        }
        Ok(())
    }
}

