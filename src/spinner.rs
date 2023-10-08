use crossterm::style::Print;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};
use futures::Future;
use std::io::stderr;
use tokio::sync::oneshot::{self, error::TryRecvError, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time;

const LOADING_SPINNER_DELAY: u64 = 40;
const LOADING_SPINNER_DOTS: [&str; 56] = [
    "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉",
    "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘", "⢃⠨", "⡃⢐", "⠍⡐", "⢋⠠",
    "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⠈⠩", "⠀⢙", "⠀⡙", "⠀⠩",
    "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀",
];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Spinner {
    tx: Sender<()>,
    handle: JoinHandle<Result<()>>,
}

impl Spinner {
    pub fn new() -> Self {
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(Self::spin(rx));
        Self { tx, handle }
    }

    pub async fn stop(self) -> Result<()> {
        self.tx.send(()).ok();
        self.handle.await??;
        Ok(())
    }

    async fn spin(mut rx: Receiver<()>) -> Result<()> {
        let mut dots = LOADING_SPINNER_DOTS.iter().cycle();
        terminal::enable_raw_mode()?;
        execute!(
            stderr(),
            cursor::SavePosition,
            cursor::Hide,
            terminal::Clear(ClearType::CurrentLine),
        )?;
        let mut interval = time::interval(time::Duration::from_millis(LOADING_SPINNER_DELAY));
        while rx.try_recv() == Err(TryRecvError::Empty) {
            execute!(
                stderr(),
                cursor::MoveToColumn(1),
                terminal::Clear(ClearType::CurrentLine),
                Print(dots.next().unwrap())
            )?;
            interval.tick().await;
        }
        execute!(
            stderr(),
            terminal::Clear(ClearType::CurrentLine),
            cursor::RestorePosition,
            cursor::Show,
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

pub async fn wrap_spinner<F>(future: F) -> Result<F::Output>
where
    F: Future + Send,
    F::Output: Send,
{
    let spinner = Spinner::new();
    let result = future.await;
    spinner.stop().await?;
    Ok(result)
}
