

#![feature(portable_simd)]

mod play_data;

pub use play_data::PlayData;



use std::error::Error;
use std::rc::Rc;

use slint::*;

slint::include_modules!();
#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new().unwrap();
    let first_field: ModelRc<ModelRc<i32>> = ModelRc::from(Rc::new(VecModel::from(
        (0..20)
            .map(|_| ModelRc::from(Rc::new(VecModel::from(vec![ 1; 10]))))
            .collect::<Vec<_>>(),
    )));

    println!("result length:");
    ui.set_field(first_field);
    ui.run()?;
    Ok(())
}
 