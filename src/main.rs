mod libs;
pub mod types;

use libs::*;
use std::error::Error;
pub use types::*;

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(new_surfs) = scrape_surfs() {
        send_email(new_surfs)?;
    }

    Ok(())
}
