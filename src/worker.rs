use crate::{
    hardware::Keyboard,
    lighting::{position, render, Frame, Press, Settings},
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::{
        mpsc::{self, Receiver, Sender, SyncSender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
pub static CAPTURE: AtomicBool = AtomicBool::new(false);
pub enum Command {
    Connect,
    Start,
    Stop,
    Disconnect,
    Exit,
}
pub struct Status {
    pub connected: bool,
    pub running: bool,
    pub message: String,
    pub frame: Frame,
    pub fps: u32,
    pub close_error: Option<String>,
}
impl Default for Status {
    fn default() -> Self {
        Self {
            connected: false,
            running: false,
            message: "Klavye bağlı değil. Önce eski uygulamayı ve ATK HUB'ı kapatın.".into(),
            frame: [[0; 3]; 90],
            fps: 0,
            close_error: None,
        }
    }
}
pub struct Worker {
    pub command: Sender<Command>,
    pub keys: SyncSender<(u32, Instant)>,
    pub settings: Arc<Mutex<Settings>>,
    pub status: Arc<Mutex<Status>>,
    join: Mutex<Option<JoinHandle<()>>>,
}
impl Worker {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let (keys, key_rx) = mpsc::sync_channel(64);
        let settings = Arc::new(Mutex::new(crate::preferences::load()));
        let status = Arc::new(Mutex::new(Status::default()));
        let (s, t) = (settings.clone(), status.clone());
        let join = thread::spawn(move || run(rx, key_rx, s, t));
        Self {
            command: tx,
            keys,
            settings,
            status,
            join: Mutex::new(Some(join)),
        }
    }
    pub fn shutdown(&self) {
        CAPTURE.store(false, Ordering::Relaxed);
        let _ = self.command.send(Command::Exit);
        if let Some(join) = self.join.lock().unwrap().take() {
            let _ = join.join();
        }
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.shutdown();
    }
}
fn message(status: &Arc<Mutex<Status>>, text: impl Into<String>, connected: bool, running: bool) {
    let mut state = status.lock().unwrap();
    state.message = text.into();
    state.connected = connected;
    state.running = running;
}
fn blackout(device: &Option<Keyboard>) -> Result<(), String> {
    if let Some(d) = device {
        d.black()
    } else {
        Ok(())
    }
}
fn run(
    rx: Receiver<Command>,
    keys: Receiver<(u32, Instant)>,
    settings: Arc<Mutex<Settings>>,
    status: Arc<Mutex<Status>>,
) {
    let mut device: Option<Keyboard> = None;
    let mut running = false;
    let mut presses = Vec::new();
    let epoch = Instant::now();
    let tick = Duration::from_nanos(16_666_667);
    let mut next = Instant::now();
    let mut last = next;
    let mut phase = 0.;
    let mut frames = 0;
    let mut measure = next;
    let mut last_effect = 0;
    loop {
        let timeout = if running {
            next.saturating_duration_since(Instant::now())
        } else {
            Duration::from_millis(100)
        };
        let cmd = match rx.recv_timeout(timeout) {
            Ok(c) => Some(c),
            Err(mpsc::RecvTimeoutError::Timeout) => None,
            Err(_) => Some(Command::Exit),
        };
        if let Some(cmd) = cmd {
            match cmd {
                Command::Connect => {
                    running = false;
                    CAPTURE.store(false, Ordering::Relaxed);
                    let _ = blackout(&device);
                    device = None;
                    message(&status, "USB klavye aranıyor…", false, false);
                    match Keyboard::connect() {
                        Ok(d) => {
                            message(
                                &status,
                                format!(
                                    "Bağlandı · Firmware {}.{:02} · USB 373b:{:04x}",
                                    d.firmware / 100,
                                    d.firmware % 100,
                                    d.pid
                                ),
                                true,
                                false,
                            );
                            device = Some(d);
                        }
                        Err(e) => message(&status, e, false, false),
                    }
                }
                Command::Start => {
                    if let Some(d) = &device {
                        match d.start() {
                            Ok(()) => {
                                running = true;
                                presses.clear();
                                while keys.try_recv().is_ok() {}
                                last = Instant::now();
                                next = last;
                                frames = 0;
                                measure = last;
                                message(
                                    &status,
                                    "Efekt çalışıyor · Pencere küçültülse de gönderim devam eder.",
                                    true,
                                    true,
                                );
                            }
                            Err(e) => message(&status, e, true, false),
                        }
                    }
                }
                Command::Stop | Command::Disconnect => {
                    running = false;
                    CAPTURE.store(false, Ordering::Relaxed);
                    presses.clear();
                    let result = blackout(&device);
                    let disconnect = matches!(cmd, Command::Disconnect);
                    if disconnect {
                        device = None;
                    }
                    let text = match result {
                        Ok(()) => "Özel efekt durduruldu · Kontrol klavyeye bırakıldı.".into(),
                        Err(e) => format!("Kapatma gönderilemedi: {e}"),
                    };
                    {
                        let mut state = status.lock().unwrap();
                        state.frame = [[0; 3]; 90];
                        state.fps = 0;
                    }
                    message(&status, text, device.is_some(), false);
                }
                Command::Exit => {
                    CAPTURE.store(false, Ordering::Relaxed);
                    let result = blackout(&device);
                    let mut state = status.lock().unwrap();
                    state.running = false;
                    state.frame = [[0; 3]; 90];
                    state.close_error = result.err();
                    break;
                }
            }
        }
        let s = settings.lock().unwrap().clone();
        CAPTURE.store(running && s.effect == 0, Ordering::Relaxed);
        if s.effect != last_effect {
            presses.clear();
            last_effect = s.effect;
        }
        while let Ok((scan, when)) = keys.try_recv() {
            if running && s.effect == 0 {
                if let Some((x, y)) = position(scan) {
                    if presses.len() == 32 {
                        presses.remove(0);
                    }
                    presses.push(Press {
                        x,
                        y,
                        time: when.saturating_duration_since(epoch).as_secs_f32(),
                    });
                }
            }
        }
        if !running {
            continue;
        }
        let now = Instant::now();
        if now < next {
            continue;
        }
        phase += (now - last).as_secs_f32() * s.speed.clamp(0.2, 3.);
        last = now;
        let seconds = now.duration_since(epoch).as_secs_f32();
        presses.retain(|p| seconds - p.time < 11.);
        let frame = render(&s, phase, seconds, &presses);
        let result = device.as_ref().unwrap().write(&frame);
        match result {
            Ok(()) => {
                frames += 1;
                let mut state = status.lock().unwrap();
                state.frame = frame;
                if measure.elapsed() >= Duration::from_secs(1) {
                    state.fps = (frames as f32 / measure.elapsed().as_secs_f32()).round() as u32;
                    frames = 0;
                    measure = Instant::now();
                }
            }
            Err(e) => {
                running = false;
                CAPTURE.store(false, Ordering::Relaxed);
                let _ = blackout(&device);
                device = None;
                {
                    let mut state = status.lock().unwrap();
                    state.frame = [[0; 3]; 90];
                    state.fps = 0;
                }
                message(
                    &status,
                    format!("USB gönderimi durdu: {e}. Yeniden bağlayın."),
                    false,
                    false,
                );
            }
        }
        // Never queue catch-up frames. USB has one owner and one frame in flight.
        next += tick;
        if next <= Instant::now() {
            next = Instant::now() + tick;
        }
    }
}
