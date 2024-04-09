


use std::f32::consts::PI;

//= Imports
use pleroma::{
	files::compression::CompressionType,
	structures::{
		color::*,
		image::*,
		matrix::*,
		rectangle,
		vectors::*,
	},
};


//= Tests

///Temp
#[test]
fn compression() {
	let data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_vec();
	let no_comp = CompressionType::None;
	let deflate = CompressionType::Deflate;
	let zlib = CompressionType::ZLib;
	let gz = CompressionType::GZ;

	println!("Before compression: [{:?}]",data.len());

	let no_comp_comp = no_comp.compress(data.clone(), pleroma::files::compression::CompressionLevel::Best);
	let no_comp_decomp = no_comp.decompress(no_comp_comp.clone());
	println!(" No compression: [{:?}] : [{:?}]",no_comp_comp.len(),no_comp_decomp.len());
	assert_eq!(data, no_comp_decomp);

	let deflate_comp = deflate.compress(data.clone(), pleroma::files::compression::CompressionLevel::Best);
	let deflate_decomp = deflate.decompress(deflate_comp.clone());
	println!(" Deflate: [{:?}] : [{:?}]",deflate_comp.len(),deflate_decomp.len());
	assert_eq!(data, deflate_decomp);

	let zlib_comp = zlib.compress(data.clone(), pleroma::files::compression::CompressionLevel::Best);
	let zlib_decomp = zlib.decompress(zlib_comp.clone());
	println!(" Zlib: [{:?}] : [{:?}]",zlib_comp.len(),zlib_decomp.len());
	assert_eq!(data, zlib_decomp);

	let gz_comp = gz.compress(data.clone(), pleroma::files::compression::CompressionLevel::Best);
	let gz_decomp = gz.decompress(gz_comp.clone());
	println!(" Gz: [{:?}] : [{:?}]",gz_comp.len(),gz_decomp.len());
	assert_eq!(data, gz_decomp);
}

/// Color
#[test]
fn color() {

	//* Into / From: Vector3 */
	assert_eq!(Vector3{x: 300.0, y: 1.0, z: 1.0}, MAGENTA.into(), "Conversion to HSV not accurate.");
	assert_eq!(Color::from(Vector3{x: 300.0, y: 1.0, z: 1.0}), MAGENTA, "Conversion from HSV not accurate.");
	//* Into / From: Vector4 */
	assert_eq!(Vector4{x: 1.0, y: 0.0, z: 1.0, w: 1.0}, MAGENTA.into(), "Color normalization innaccurate.");
	assert_eq!(Color::from(Vector4{x: 1.0, y: 0.0, z: 1.0, w: 1.0}), MAGENTA, "Color from normalization innaccurate.");

	//* Fade */
	assert_eq!(BLANK, BLACK.fade(0.0), "Failed to fade color.");
	//* To Int */
	assert_eq!(WHITE.to_int(), -1, "Failed to convert to int.");
	//* Tint */
	assert_eq!(WHITE.tint(BLACK), BLACK, "Failed to properly tint.");
	//* Brightness */
	assert_eq!(BLACK.brightness(0.509804), GRAY, "Failed to properly apply brightness.");
	//* Contrast */
	assert_eq!(RED.contrast(0.5), Color{r: 255, g: 0, b: 0, a: 255}, "Failed to properly apply contrast.");
	//* Alpha */
	assert_eq!(BLACK.alpha(0.0), BLANK, "Failed to properly alpha.");
	//* Alpha blend */
	assert_eq!(WHITE.alpha_blend(RED, WHITE), RED, "Failed to properly tint.");
	//* Hex */
	assert_eq!(WHITE, Color::hex(4294967295), "Failed to convert from Hex.");
	
}

/// Rectangle
// TODO
#[test]
fn rectangle() {
	//* Zero */
	assert_eq!(rectangle::ZERO, rectangle::Rectangle{ x: 0.0, y: 0.0, width: 0.0, height: 0.0 });
}

/// Vector2
#[test]
fn vector_2() {

	//* Negate */
	assert_eq!(!ONE_2, Vector2{x: -1.0, y: -1.0}, "Negate failed.");
	//* Length */
	assert_eq!(ONE_2.length(), 1.4142135, "Length failed.");
	//* Square length */
	assert_eq!(ONE_2.length_sqr(), 2.0, "Length squared failed.");
	//* Dot Product */
	assert_eq!(ONE_2.dot_product(ONE_2), 2.0, "Dot product failed.");
	//* Distance */
	assert_eq!(ZERO_2.distance(ONE_2), 1.4142135, "Distance failed.");
	//* Square distance */
	assert_eq!(ZERO_2.distance_sqr(ONE_2), 2.0, "Distance squared failed.");
	//* Angle */
	assert_eq!((!ONE_2).angle(ONE_2) * -(360.0 / PI), 180.0, "Angle failed.");
	//* Angle line */
	assert_eq!((ONE_2).angle_line(TEN_2) * -(360.0 / PI), 90.0, "Angle line failed.");
	//* Normalize */
	assert_eq!(ONE_2.normalize(), Vector2{x: 0.70710677, y: 0.70710677}, "Normalize failed.");
	//* Transform */
	// TODO Check
	assert_eq!((ONE_2).transform(IDENTITY), ONE_2, "Transform failed.");
	//* Lerp */
	assert_eq!(ZERO_2.lerp(ONE_2, 0.5), Vector2{x: 0.5, y: 0.5}, "Lerp failed.");
	//* Reflect */
	// TODO Check
	assert_eq!((TEN_2).reflect(ONE_2), Vector2{ x: -30.0, y: -30.0 }, "Reflect failed.");
	//* Rotate */
	assert_eq!(ONE_2.rotate(180.0_f32.to_radians()), Vector2{x: -1.0, y: -1.0}, "Rotate failed.");
	//* Move towards */
	assert_eq!(ZERO_2.move_towards(ONE_2, 0.5), Vector2{x: 0.35355338, y: 0.35355338}, "Move towards failed.");
	//* Invert */
	assert_eq!(TEN_2.invert(), Vector2{x: 0.1, y: 0.1}, "Invert failed.");
	//* Clamp */
	assert_eq!((!TEN_2).clamp(ZERO_2, ONE_2), ZERO_2, "Minimum clamp failed.");
	assert_eq!(TEN_2.clamp(ZERO_2, ONE_2), ONE_2, "Maximum clamp failed.");
	//* Clamp magnitude */
	assert_eq!((!TEN_2).clamp_mag(0.0, 1.0), !Vector2{x: 0.7071068, y: 0.7071068}, "Minimum clamp magnitude failed.");
	assert_eq!(TEN_2.clamp_mag(0.0, 1.0), Vector2{x: 0.7071068, y: 0.7071068}, "Maximum clamp magnitude failed.");

}
/// Vector3
#[test]
fn vector_3() {

	//* Cross product */
	assert_eq!(ONE_3.cross_product(TEN_3), ZERO_3, "Cross product failed.");
	//* Perpendicular */
	assert_eq!(ONE_3.perpendicular(), Vector3{ x: 0.0, y: 1.0, z: -1.0 }, "Perpendicular failed.");
	//* Length */
	assert_eq!(TEN_3.length(), 17.32050807, "Length failed.");
	//* Length square */
	assert_eq!(TEN_3.length_sqr(), 300.0, "Length square failed.");
	//* Dot product */
	assert_eq!(TEN_3.dot_product(ONE_3), 30.0, "Dot product failed.");
	//* Distance */
	assert_eq!(TEN_3.distance(ONE_3), 15.588457268, "Distance failed.");
	//* Distance square */
	assert_eq!(TEN_3.distance_sqr(ONE_3), 243.0, "Distance square failed.");
	//* Angle */
	assert_eq!(ONE_3.angle(TEN_3), 0.0, "Angle failed.");
	//* Negate */
	assert_eq!(!TEN_3, Vector3{ x: -10.0, y: -10.0, z: -10.0 }, "Negate failed.");
	//* Normalize */
	assert_eq!(ONE_3.normalize(), Vector3{x: 0.57735026, y: 0.57735026, z: 0.57735026}, "Normalize failed.");
	//* Orthonormalize */
	// TODO Check
	// TODO What even is this for?
	//* Transform */
	// TODO Check
	// TODO
	//* Rotate by quaternion */
	// TODO Check
	// TODO
	//* Rotate by axis */
	// TODO Check
	// TODO
	//* Move towards */
	assert_eq!(ZERO_3.move_towards(TEN_3, 2.0), Vector3{ x: 1.1547005, y: 1.1547005, z: 1.1547005 }, "Move towards failed.");
	//* Lerp */
	assert_eq!(ZERO_3.lerp(TEN_3, 0.1), ONE_3, "Lerp failed.");
	//* Reflect */
	// TODO Check
	// TODO
	//* Min */
	assert_eq!(ONE_3.min(ZERO_3), ZERO_3, "Minimum failed.");
	//* Max */
	assert_eq!(ONE_3.max(ZERO_3), ONE_3, "Maximum failed.");
	//* Barycenter */
	// TODO Check
	// TODO
	//* Unproject */
	// TODO Check
	// TODO
	//* Invert */
	assert_eq!(TEN_3.invert(), Vector3{ x: 0.1, y: 0.1, z: 0.1 }, "Inversion failed.");
	//* Clamp */
	assert_eq!((!TEN_3).clamp(ZERO_3, ONE_3), ZERO_3, "Minimum clamp failed.");
	assert_eq!(TEN_3.clamp(ZERO_3, ONE_3), ONE_3, "Maximum clamp failed.");
	//* Clamp Value */
	assert_eq!((!TEN_3).clamp_value(0.0, 1.0), !Vector3{x: 0.5773502, y: 0.5773502, z: 0.5773502}, "Minimum clamp magnitude failed.");
	assert_eq!(TEN_3.clamp_value(0.0, 1.0), Vector3{x: 0.5773502, y: 0.5773502, z: 0.5773502}, "Maximum clamp magnitude failed.");
	//* Refract */
	// TODO Check
	// TODO

}
/// Vector4
// TODO
#[test]
fn vector_4() {

	//* Length */
	assert_eq!(TEN_4.length(), 20.0, "Length failed.");
	//* Length square */
	assert_eq!(TEN_4.length_square(), 400.0, "Length square failed.");
	//* Dot product */
	assert_eq!(ONE_4.dot_product(ONE_4), 4.0, "Dot product failed.");
	//* Distance */
	assert_eq!(ONE_4.distance(ZERO_4), 2.0, "Distance failed.");
	//* Distance square */
	assert_eq!(ONE_4.distance_square(ZERO_4), 4.0, "Distance square failed.");
	//* Normalize */
	assert_eq!(ONE_4.normalize(), Vector4{ x: 0.5, y: 0.5, z: 0.5, w: 0.5 }, "Normalize failed.");
	//* Min */
	assert_eq!(ONE_4.min(ZERO_4), ZERO_4, "Minimum failed.");
	//* Max */
	assert_eq!(ONE_4.max(ZERO_4), ONE_4, "Maximum failed.");
	//* Lerp */
	assert_eq!(ZERO_4.lerp(TEN_4, 0.1), ONE_4, "Lerp failed.");
	//* Move towards */
	assert_eq!(ZERO_4.move_towards(TEN_4, 2.0), ONE_4, "Move towards failed.");
	//* Invert */
	assert_eq!(TEN_4.invert(), Vector4{ x: 0.1, y: 0.1, z: 0.1, w: 0.1 }, "Inversion failed.");

}

/// Font
// TODO
#[test]
fn font() {}

/// Image
// TODO
#[test]
fn image() {
	//unsafe {
		unsafe { SetTraceLogLevel(7) }

		//* Testing PartialEq */
		let partial_1 = Image::load("data/test_1.png");
		let partial_2 = Image::load("data/test_2.png");
		assert_eq!(partial_1, partial_1, "PartialEq is not working. (The same file is not equal to itself)");
		assert_ne!(partial_1, partial_2, "PartialEq is not working. (Two different files are equal)");
		partial_1.unload();
		partial_2.unload();

		//* Load Raw */
		// TODO

		//* Load SVG */
		// TODO

		//* Load Animation */
		// TODO

		//* Load from memory */
		// TODO

		//* Load from texture */
		// TODO

		//* Load from screen */
		// TODO

		//* Is image ready */
		// TODO

		//* Export to file */
		// TODO

		//* Export to memory */
		// TODO

		//* Export as code */
		// TODO

		//* Generate plain color */
		// TODO

		//* Generate linear gradient */
		// TODO

		//* Generate radial gradient */
		// TODO

		//* Generate square grandient */
		// TODO

		//* Generate checked */
		// TODO

		//* Generate white noise */
		// TODO

		//* Generate perlin noise */
		// TODO

		//* Generate celluar algorithm */
		// TODO

		//* Generate grayscale text */
		// TODO

		//* Copy image */
		// TODO

		//* Image subset */
		// TODO

		//* Image Text */
		// TODO

		//* Image text ex */
		// TODO

		//* Change format */
		// TODO

		//* Convert to power of 2 */
		// TODO

		//* Crop image */
		// TODO

		//* Crop image based on alpha */
		// TODO

		//* Clear alpha to specified color */
		// TODO

		//* Apply alpha mask */
		// TODO

		//* Premultiply alpha channel */
		// TODO

		//* Apply gaussian blur */
		// TODO

		//* Resize image bicubic */
		// TODO

		//* Resize image nearest-neighbor */
		// TODO

		//* Resize canvas */
		// TODO

		//* Compute mipmaps */
		// TODO

		//* Dither */
		// TODO

		//* Flip vertical */
		// TODO

		//* Flip horizontal */
		// TODO

		//* Rotate */
		// TODO

		//* Rotate 90 degrees clockwise */
		// TODO

		//* Rotate 90 degrees counter clockwise */
		// TODO

		//* Tint color */
		// TODO

		//* Invert color */
		// TODO

		//* Grayscale */
		// TODO

		//* Set contrast */
		// TODO

		//* Set brightness */
		// TODO

		//* Replace color */
		// TODO

		//* Load palette */
		// TODO

		//* Get alpha border rectangle */
		// TODO

		//* Get image color at position */
		// TODO

		//* Clear background */
		// TODO

		//* Draw pixel */
		// TODO

		//* Draw pixel using vector */
		// TODO

		//* Draw line */
		// TODO

		//* Draw line using vectors */
		// TODO

		//* Draw circle */
		// TODO

		//* Draw circle using vector */
		// TODO

		//* Draw circle outline */
		// TODO

		//* Draw circle outline using vector */
		// TODO

		//* Draw rectangle */
		// TODO

		//* Draw rectangle using vector */
		// TODO

		//* Draw rectangle using rectangle */
		// TODO

		//* Draw rectangle lines */
		// TODO

		//* Draw image */
		// TODO

		//* Draw image text */
		// TODO

		//* Draw image text ex */
		// TODO

		//* To texture */
		// TODO

		//* Tu cubemap */
		// TODO
	//}
}

/// Matrix
// TODO
#[test]
fn matrix() {}

/// RenderTexture
// TODO
#[test]
fn render_texture() {}

/// Texture
// TODO
#[test]
fn texture() {}


extern "C" { fn SetTraceLogLevel(logLevel: i32); }