pub mod ui;
pub mod func;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ui::run()
}