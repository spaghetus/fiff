use fiff::FromFiff;

#[test]
fn decode() {
	let data = include_bytes!("test.fiff");
	let image = fiff::Image::from_fiff(&mut std::io::Cursor::new(data)).unwrap();
	println!("{}", image.blocks.len());
}
