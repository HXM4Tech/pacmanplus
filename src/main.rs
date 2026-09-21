use pacmanplus::run;
use pacmanplus::clean::clean_pacman_download_dirs;
use std::process::exit;
use std::thread;

#[tokio::main]
async fn main() {
    let mut signals = signal_hook::iterator::Signals::new(&[
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGQUIT,
    ]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            let _ = clean_pacman_download_dirs();
            exit(128 + sig);
        }
    });

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let ret = run(&args).await;

    exit(ret);
}
