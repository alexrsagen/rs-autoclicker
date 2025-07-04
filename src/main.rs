mod alloc;
mod args;
mod bytesize;
mod config;
mod key;
mod logger;

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use anyhow::{Result, anyhow};
use crossbeam_channel::bounded;

#[global_allocator]
static GLOBAL: alloc::SystemTrackingAllocator = alloc::SystemTrackingAllocator::new_system();

fn main() -> Result<()> {
    // parse command-line arguments
    let args = args::Args::parse();

    // set up logger
    logger::try_init(args.log_level)?;

    // load or create default config
    let config = config::Config::load_or_init(&args.config_path)?;

    let click_delay = config
        .clicks_per_second
        .map(|cps| Duration::from_secs_f64(1.0 / cps));

    // create a channel which can tell the clicking to stop
    let (click_send, click_recv) = bounded::<bool>(1);

    // spawn a thread in which to click
    thread::spawn(move || {
        loop {
            // wait for message (blocking)
            if let Ok(click) = click_recv.recv() {
                if !click {
                    continue;
                }
            }

            loop {
                // click
                let _ = rdev::simulate(&rdev::EventType::ButtonPress(rdev::Button::Left));
                let _ = rdev::simulate(&rdev::EventType::ButtonRelease(rdev::Button::Left));

                if let Some(click_delay) = &click_delay {
                    thread::sleep(*click_delay);
                }
                // check for message (non-blocking)
                if let Ok(click) = click_recv.try_recv() {
                    if !click {
                        break;
                    }
                }
            }
        }
    });

    // listen for input events
    let clicking = AtomicBool::new(false);
    rdev::listen(move |event| {
        let (pressed, released) = if let rdev::EventType::ButtonPress(event_button) = event.event_type
            && let key::Key::Mouse { button } = config.keybind
            && button == event_button
        {
            (true, false)
        } else if let rdev::EventType::ButtonRelease(event_button) = event.event_type
            && let key::Key::Mouse { button } = config.keybind
            && button == event_button
        {
            (false, true)
        } else if let rdev::EventType::KeyPress(event_key) = event.event_type
            && let key::Key::Keyboard { key } = config.keybind
            && key == event_key
        {
            (true, false)
        } else if let rdev::EventType::KeyRelease(event_key) = event.event_type
            && let key::Key::Keyboard { key } = config.keybind
            && key == event_key
        {
            (false, true)
        } else {
            (false, false)
        };

        let current_state = clicking.load(Ordering::SeqCst);
        let desired_state = if config.toggle && released {
            !current_state
        } else if !config.toggle {
            pressed || current_state && !released
        } else {
            current_state
        };
        if desired_state != current_state {
            if let Ok(()) = click_send.send(desired_state) {
                clicking.store(desired_state, Ordering::SeqCst);
            }
        }
    })
    .map_err(|e| anyhow!("{e:?}"))?;

    Ok(())
}
