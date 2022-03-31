use array2::Array2;
use array2b::Array2b;
use csc411_image::{Read, Write, RgbImage};
use clap::{Command, ArgGroup};

fn main() {

	// get command line arguments and parse them using clap

	let matches = get_command_line_matches();
	parse_matches(matches);

}

// utilizing clap crate to grab the command line arguments present 
fn get_command_line_matches() -> clap::ArgMatches {	
	
	// match upon the possible arguments using clap
	Command::new("Image Transformer")
		.version("1.0")
		.author("Arlen Dumas\nConnor Gray")
		.about("Proper usage: <Image path> <storage option> <transformation>")
		.args([
			clap::arg!(-c --"column-major"),
			clap::arg!(-r --"row-major"),
			clap::arg!(-b --"block-major"),
			clap::arg!(-f --flip [DIRECTION])
				.possible_values(&["vertical", "horizantal"]),
			clap::arg!(-o --rotate [DEGREE])
				.possible_values(&["0", "90", "180", "270"]),
			clap::arg!(-t --transpose),
			clap::arg!([INPUT]),
			clap::arg!([OUTPUT])
			])
		.group(ArgGroup::new("storage_method")
			// storage options
			.args(&["row-major", "column-major", "block-major"])
			.required(true))
		.group(ArgGroup::new("transform_type")
			.args(&["flip", "rotate", "transpose"])
			.required(true))
		.get_matches()
}

// pass in the command line arguments to 
fn parse_matches(matches: clap::ArgMatches) {
	let input_file: Option<&str>;
	let output_file: Option<&str>;
	let storage_method: &str;
	let output_image: csc411_image::RgbImage;

	// I/O matches 
	if matches.is_present("INPUT") {
		input_file = matches.value_of("INPUT");
	}
	else {
		input_file = None;
	}

	if matches.is_present("OUTPUT") {
		output_file = matches.value_of("OUTPUT");
	}
	else {
		output_file = None;
	}

	// storage matches
	if matches.is_present("row-major") {
		storage_method = "rm";
	} 
	else if matches.is_present("column-major") {
		storage_method = "cm";
	} 
	else { // block major
		storage_method = "bm";
	}

	// transform matches
	if matches.is_present("rotate") {
		let degree = matches.value_of("rotate").unwrap();
		output_image = transform(input_file, storage_method, &["r", degree].join(""));
	}
	else if matches.is_present("flip") {
		let direction = matches.value_of("flip").unwrap();
		output_image = transform(input_file, storage_method, &["f_", direction].join(""));
	}
	else { // transform
		output_image = transform(input_file, storage_method, "t");
	}
	
	output_image.write(output_file);
}

// handle the transform based on storage method
pub fn transform(input_file: Option<&str>, storage_method: &str, effect: &str) -> csc411_image::RgbImage {
	let image = RgbImage::read(input_file).unwrap(); 

    let w = image.width as usize;
    let h = image.height as usize;
    let new_w: usize;
    let new_h: usize;

    if effect == "r90" || effect == "r270" || effect == "t" {
    	new_w = image.height as usize;
    	new_h = image.width as usize;
    } else {
    	new_w = image.width as usize;
    	new_h = image.height as usize;
    }

    let destination = match storage_method {
    	"rm" => transf(Array2::from_row_major(w, h, image.pixels).iter_row_major(), effect, new_w, new_h),
    	"cm" => transf(Array2::from_row_major(w, h, image.pixels).iter_col_major(), effect, new_w, new_h),
    	"bm" => transf(Array2b::from_row_major_16k_block(w, h, image.pixels).iter(), effect, new_w, new_h),
    	&_   => panic!();
    };

    // store the new image
	RgbImage {
		pixels: destination,
		width: new_w as u32,
		height: new_h as u32,
		denominator: image.denominator
	}
}
// executes the transformations with row/column/or block major iterator as input
pub fn transf<'a, I>(iter: I, effect: &str, w: usize, h: usize) -> Vec<csc411_image::Rgb>
	where I: Iterator<Item = (usize, usize, &'a csc411_image::Rgb)>,
{
	let c_r_func = get_transform_function(w, h, effect);
	let mut destination = vec![csc411_image::Rgb { red: 0, green: 0, blue: 0 }; w * h];
	let mut index: usize;

	for tuple in iter {	
		let c_r = c_r_func(tuple.0, tuple.1);
		index = c_r.1 * w + c_r.0;
		destination[index].red   = tuple.2.red;
		destination[index].green = tuple.2.green;
		destination[index].blue  = tuple.2.blue;
	}
	destination
}

fn get_transform_function(w: usize, h: usize, effect: &str) -> Box<dyn Fn(usize, usize) -> (usize, usize)> {
	match effect {
		"r0"           => Box::new(move |c, r| (c, r)),
		"r90"          => Box::new(move |c, r| (w - r - 1, c)),
		"r180"         => Box::new(move |c, r| (w - c - 1, h - r - 1)),
		"r270"         => Box::new(move |c, r| (r, h - c - 1)),
		"f_horizantal" => Box::new(move |c, r| (w - c - 1, r)),
		"f_vertical"   => Box::new(move |c, r| (c, h - r - 1)),
		"t"            => Box::new(move |c, r| (r, c)),
		_              => Box::new(move |c, r| (c, r)),
	}
}
