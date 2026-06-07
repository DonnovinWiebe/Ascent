#![allow(clippy::elidable_lifetime_names)]
#![windows_subsystem = "windows"] // I follow the lifetime notation/elision suggestions in my editor (Zed).

use std::sync::atomic::{self, AtomicBool};
use crate::{iced_ui::state::app::App, vault::schrod::Schrod::{self, Pass}};
pub mod vault;
pub mod iced_ui;

// flags for swapping the ui
const SWAP_UI_FLAG: &str = "ASCENT_SWAP_UI";
static SWAP_REQUESTED: AtomicBool =  AtomicBool::new(false);

fn main() -> Schrod<()> {
    // the loop that allows Ascent to swap what ui is being used
    loop {
        // checking if Ratatui has been requested and clearing the swap flag
        let cook_with_remmy = std::env::args().any(|a| a == "--tui") || std::env::var(SWAP_UI_FLAG).as_deref() == Ok("tui");
        unsafe { std::env::remove_var(SWAP_UI_FLAG); }

        // sets the iced backend on linux manually - see below
        #[cfg(target_os = "linux")]
        if !cook_with_remmy {
            // there have been some rendering issues on Fedora, and this fixed it
            unsafe { std::env::set_var("WGPU_BACKEND", "gl"); }
        }

        // runs the app in the requested ui and checks if a swap was requested after running
        let requested_swap = {
            // the Ratatui ui
            if cook_with_remmy {
                todo!()
            }

            // the Iced ui
            else {
                let run_result = Schrod::from_result(
                    iced::application(App::new, App::update, App::view)
                        .theme(App::theme)
                        .subscription(App::subscription)
                        .run(),
                    "Ascent encountered an error.", "main()"
                );
                if run_result.is_fail() { return run_result }
                SWAP_REQUESTED.load(atomic::Ordering::SeqCst)
            }
        };

        // exits if no swap was requested
        if !requested_swap { break; }

        // sets the ui flag to the desired ui
        let new_ui_flag = if cook_with_remmy { "iced" } else { "tui" };
        unsafe { std::env::set_var(SWAP_UI_FLAG, new_ui_flag); }
    }

    // exits cleanly
    Pass(())
}