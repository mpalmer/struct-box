use strong_box::{generate_key, StaticStrongBox};

mod sub {
	use serde::{Deserialize, Serialize};
	use struct_box::StructBox;

	#[derive(Serialize, Deserialize, StructBox)]
	pub(super) struct DemoData {
		x: u32,
		y: u32,
	}

	impl DemoData {
		pub(super) fn new(x: u32, y: u32) -> Self {
			Self { x, y }
		}

		pub(super) fn x(&self) -> u32 {
			self.x
		}
	}
}

#[test]
fn visibility() {
	let strong_key = generate_key();
	let strong_box = StaticStrongBox::new(strong_key, vec![strong_key]);

	let data = sub::DemoData::new(4, 2);

	let ciphertext = data.encrypt(&strong_box, b"encryption context").unwrap();

	let decrypted_data =
		sub::DemoData::decrypt(&ciphertext, &strong_box, b"encryption context").unwrap();

	// Technically, this test "passes" if the code compiles, but let's make this look like a
	// real test
	assert_eq!(data.x(), decrypted_data.x());
}
