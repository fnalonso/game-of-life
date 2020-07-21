use std::thread;
use std::process;
use std::time::Duration;

use ctrlc;
use structopt::StructOpt;

use game_of_life::cells::Cells;
use game_of_life::terminal;
use game_of_life::utils;


#[derive(Debug, StructOpt)]
#[structopt(name = "game_of_life")]
pub struct Opt {

    /// Number of rows
    #[structopt(short, long, default_value="30")]
    rows: i32,

    /// Number of columns
    #[structopt(short, long, default_value="60")]
    columns: i32,

    /// Board padding
    #[structopt(short, long, default_value="5")]
    padding: i32,

    /// Starting alive population percentage
    #[structopt(long, default_value="15")]
    population: f32,

    /// Refresh delay in miliseconds (The higher the slower)
    #[structopt(short, long, default_value="100")]
    delay: u64,

    /// Layout file
    #[structopt(long)]
    layout: Option<String>
}



fn main() {
    ctrlc::set_handler( move || {
        terminal::reset();
        println!("\n\nBye!");
        terminal::clear();
        process::exit(0);

    }).expect("Error while trying to set the signal handler");


    let opts = Opt::from_args();
    let mut layout_data:Option<String> = None;
    let cells;


    if let Some(filename) = opts.layout{
        println!("Layout file {} received. Loading...", filename);
        layout_data = match utils::load_layout_file(filename) {
            Ok(data) => {
                Some(data)
            },
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };
    }


    match layout_data {
        Some(data) => {
            cells = Cells::create_from_layout(data);
        },
        None => {
            cells = Cells::create_from_random(
                opts.rows, opts.columns, opts.population
            );
        }
    }

    // Populate the cells neighbours
    cells.populate_neighbours();

    // Clear the terminal before start
    terminal::clear();
    loop {
        cells.draw_cells(opts.padding);
        cells.update_population();
        println!();
        thread::sleep(Duration::from_millis(opts.delay))
    }
}
