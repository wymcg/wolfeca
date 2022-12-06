mod wolfcode;

use clap::Parser;
use image::{RgbImage, Rgb};
use crate::wolfcode::get_next_state_from_local_state;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify Wolfram code
    #[arg(short, long, required = true)]
    code: u8,

    /// Specify image width
    #[arg(short, long, default_value = "100")]
    width: u32,

    /// Specify number of steps
    #[arg(short, long, default_value = "100")]
    steps: u32,

    /// Specify filename
    #[arg(short, long)]
    filename: String,
}

fn main() {
    // get arguments from command line
    let args = Args::parse();

    // make the automata states
    let mut current_state: Vec<bool>;
    let mut next_state: Vec<bool> = Vec::new();

    // setup the current state
    for _ in 0..args.width {
        next_state.push(false);
    }

    // set the initial cell in the center
    let state_center = next_state.len() / 2;
    next_state[state_center] = true;

    // create the image
    let mut image = RgbImage::new(args.width, args.steps);

    for step in 0..args.steps {
        // move the next state to be the current state
        current_state = next_state.clone();

        // draw the current state
        for cell_idx in 0..current_state.len() {
            match current_state[cell_idx] {
                true => {image.put_pixel(cell_idx as u32, step, Rgb([0, 0, 0]))}
                false => {image.put_pixel(cell_idx as u32, step, Rgb([255, 255, 255]))}
            }
        }

        // populate the next state
        for i in 0..current_state.len() {
            let mut left: bool = false;
            let mut right: bool = false;
            let center: bool = current_state[i];

            if i > 0 {left = current_state[i - 1]}
            if i < current_state.len() - 1 {right = current_state[i + 1]}

            next_state[i] = get_next_state_from_local_state(args.code, left, center, right);
        }
    }


    image.save(args.filename);
}
