


use std::f32::consts::PI;

//= Imports
use navia::{color::{Color, *}, vectors::*};


//= Tests

/// Color
#[test]
fn color() {
	//* Conversion from ffi */
	let color_from_ffi = Color::from(raylib_ffi::colors::BLACK);
	let color_default = BLACK;
	assert_eq!(color_default, color_from_ffi, "Error in conversion from FFI.");

	//* Conversion into ffi */
	let ffi_from_struct: raylib_ffi::Color = BLACK.into();
	let ffi_default = raylib_ffi::colors::BLACK;
	let ffi_into_result = ffi_default.r == ffi_from_struct.r &&
		ffi_default.g == ffi_from_struct.g &&
		ffi_default.b == ffi_from_struct.b &&
		ffi_default.a == ffi_from_struct.a;
	assert!(ffi_into_result, "Error in conversion into FFI.");

	//* Compare each const to constructors */
	assert_eq!(LIGHTGRAY, Color::from(raylib_ffi::colors::LIGHTGRAY), "Lightgray not the same.");
	assert_eq!(GRAY, Color::from(raylib_ffi::colors::GRAY), "Gray not the same.");
	assert_eq!(DARKGRAY, Color::from(raylib_ffi::colors::DARKGRAY), "Darkgray not the same.");
	assert_eq!(YELLOW, Color::from(raylib_ffi::colors::YELLOW), "Yellow not the same.");
	assert_eq!(GOLD, Color::from(raylib_ffi::colors::GOLD), "Gold not the same.");
	assert_eq!(ORANGE, Color::from(raylib_ffi::colors::ORANGE), "Orange not the same.");
	assert_eq!(PINK, Color::from(raylib_ffi::colors::PINK), "Pink not the same.");
	assert_eq!(RED, Color::from(raylib_ffi::colors::RED), "Red not the same.");
	assert_eq!(MAROON, Color::from(raylib_ffi::colors::MAROON), "Maroon not the same.");
	assert_eq!(GREEN, Color::from(raylib_ffi::colors::GREEN), "Green not the same.");
	assert_eq!(LIME, Color::from(raylib_ffi::colors::LIME), "Lime not the same.");
	assert_eq!(DARKGREEN, Color::from(raylib_ffi::colors::DARKGREEN), "Darkgreen not the same.");
	assert_eq!(SKYBLUE, Color::from(raylib_ffi::colors::SKYBLUE), "Skyblue not the same.");
	assert_eq!(BLUE, Color::from(raylib_ffi::colors::BLUE), "Blue not the same.");
	assert_eq!(DARKBLUE, Color::from(raylib_ffi::colors::DARKBLUE), "Darkblue not the same.");
	assert_eq!(PURPLE, Color::from(raylib_ffi::colors::PURPLE), "Purple not the same.");
	assert_eq!(VIOLET, Color::from(raylib_ffi::colors::VIOLET), "Violet not the same.");
	assert_eq!(DARKPURPLE, Color::from(raylib_ffi::colors::DARKPURPLE), "Darkpurple not the same.");
	assert_eq!(BEIGE, Color::from(raylib_ffi::colors::BEIGE), "Beige not the same.");
	assert_eq!(BROWN, Color::from(raylib_ffi::colors::BROWN), "Brown not the same.");
	assert_eq!(DARKBROWN, Color::from(raylib_ffi::colors::DARKBROWN), "Darkbrown not the same.");
	assert_eq!(WHITE, Color::from(raylib_ffi::colors::WHITE), "White not the same.");
	assert_eq!(BLACK, Color::from(raylib_ffi::colors::BLACK), "Black not the same.");
	assert_eq!(BLANK, Color::from(raylib_ffi::colors::BLANK), "Blank not the same.");
	assert_eq!(MAGENTA, Color::from(raylib_ffi::colors::MAGENTA), "Magenta not the same.");
	assert_eq!(RAYWHITE, Color::from(raylib_ffi::colors::RAYWHITE), "Raywhite not the same.");
}

/// Vector2
#[test]
fn vector_2() {
	//* Conversion from ffi */
	let vector_from_ffi = Vector2::from(raylib_ffi::Vector2{x: 0.0, y: 0.0});
	let vector_default = ZERO_2;
	assert_eq!(vector_default, vector_from_ffi, "Error in conversion from FFI.");

	//* Conversion into ffi */
	let ffi_from_struct: raylib_ffi::Vector2 = ZERO_2.into();
	let ffi_default = raylib_ffi::Vector2{x: 0.0, y: 0.0};
	let ffi_into_result = ffi_default.x == ffi_from_struct.x && ffi_default.y == ffi_from_struct.y;
	assert!(ffi_into_result, "Error in conversion into FFI.");
	
	//* Negate */
	let navia_distance_sqr = !ONE_2;
	assert_eq!(navia_distance_sqr, Vector2{x: -1.0, y: -1.0}, "Failed to negate.");

	//* Length */
	let navia_length = ONE_2.length();
	assert_eq!(navia_length, 1.4142135, "Failed to calculate proper length.");

	//* Square length */
	let navia_length_sqr = ONE_2.length_sqr();
	assert_eq!(navia_length_sqr, 2.0, "Failed to calculate proper square length.");

	//* Dot Product */
	let navia_dot = ONE_2.dot_product(ONE_2);
	assert_eq!(navia_dot, 2.0, "Failed to calculate proper dot product.");

	//* Distance */
	let navia_distance = ZERO_2.distance(ONE_2);
	assert_eq!(navia_distance, 1.4142135, "Failed to calculate proper distance.");

	//* Square distance */
	let navia_distance_sqr = ZERO_2.distance_sqr(ONE_2);
	assert_eq!(navia_distance_sqr, 2.0, "Failed to calculate proper square distance.");

	//* Angle */
	let navia_angle = (!ONE_2).angle(ONE_2) * -(360.0 / PI);
	assert_eq!(navia_angle, 180.0, "Failed to calculate proper angle.");

	//* Angle line */
	// TODO

	//* Normalize */
	let navia_normalize = ONE_2.normalize();
	assert_eq!(navia_normalize, Vector2{x: 0.70710677, y: 0.70710677}, "Failed to normalize.");

	//* Transform */
	// TODO

	//* Lerp */
	let navia_lerp = ZERO_2.lerp(ONE_2, 0.5);
	assert_eq!(navia_lerp, Vector2{x: 0.5, y: 0.5}, "Failed to properly lerp.");

	//* Reflect */
	// TODO

	//* Rotate */
	let navia_rotate = ONE_2.rotate(180.0_f32.to_radians());
	assert_eq!(navia_rotate, Vector2{x: -1.0, y: -1.0}, "Failed to properly rotate.");

	//* Move towards */
	let navia_move = ZERO_2.move_towards(ONE_2, 0.5);
	assert_eq!(navia_move, Vector2{x: 0.35355338, y: 0.35355338}, "Failed to properly move towards.");

	//* Invert */
	let navia_invert = Vector2{x: 10.0, y: 10.0}.invert();
	assert_eq!(navia_invert, Vector2{x: 0.1, y: 0.1}, "Failed to properly invert.");

	//* Clamp */
	let navia_clamp_min = Vector2{x: -10.0, y: -10.0}.clamp(ZERO_2, ONE_2);
	assert_eq!(navia_clamp_min, ZERO_2, "Failed to properly clamp minimum value.");
	let navia_clamp_max = Vector2{x: 10.0, y: 10.0}.clamp(ZERO_2, ONE_2);
	assert_eq!(navia_clamp_max, ONE_2, "Failed to properly clamp maximum value.");

	//* Clamp magnitude */
	let navia_clamp_mag_min = Vector2{x: -10.0, y: -10.0}.clamp_mag(0.0, 1.0);
	assert_eq!(navia_clamp_mag_min, !Vector2{x: 0.70710678, y: 0.70710678}, "Failed to properly clamp minimum value.");
	let navia_clamp_mag_max = Vector2{x: 10.0, y: 10.0}.clamp_mag(0.0, 1.0);
	assert_eq!(navia_clamp_mag_max, Vector2{x: 0.70710678, y: 0.70710678}, "Failed to properly clamp maximum value.");

}
/// Vector3
#[test]
fn vector_3() {

}
/// Vector4
#[test]
fn vector_4() {

}

/// Matrix
#[test]
fn matrix() {

}

//* Mesh */
//#[test]
//fn mesh_conversion() {
//	unsafe {
//		raylib_ffi::SetTraceLogLevel(raylib_ffi::enums::TraceLogLevel::None as i32);
//		raylib_ffi::InitWindow(1280, 720, raylib_ffi::rl_str!("mesh_conversion"));
//
//		let non_ffi_mesh = Mesh::gen_cube(1.0,1.0,1.0);
//		//let ffi_mesh = raylib_ffi::GenMeshCube(1.0, 1.0, 1.0);
//		let conv_ffi = Mesh::from_ffi(non_ffi_mesh.into_ffi());
//
//		assert_eq!(non_ffi_mesh.vertices, conv_ffi.vertices, "\n\tThe Vertices are dissimilar\n");
//		assert_eq!(non_ffi_mesh.texcoords, conv_ffi.texcoords, "\n\tThe Texcoords are dissimilar\n");
//		assert_eq!(non_ffi_mesh.texcoords2, conv_ffi.texcoords2, "\n\tThe Texcoord2s are dissimilar\n");
//		assert_eq!(non_ffi_mesh.normals, conv_ffi.normals, "\n\tThe Normals are dissimilar\n");
//		assert_eq!(non_ffi_mesh.tangents, conv_ffi.tangents, "\n\tThe Tangents are dissimilar\n");
//		assert_eq!(non_ffi_mesh.colors, conv_ffi.colors, "\n\tThe Colors are dissimilar\n");
//		assert_eq!(non_ffi_mesh.indices, conv_ffi.indices, "\n\tThe Indices are dissimilar\n");
//		assert_eq!(non_ffi_mesh.anim_vertices, conv_ffi.anim_vertices, "\n\tThe AnimVertices are dissimilar\n");
//		assert_eq!(non_ffi_mesh.anim_normals, conv_ffi.anim_normals, "\n\tThe AnimNormals are dissimilar\n");
//		assert_eq!(non_ffi_mesh.bone_ids, conv_ffi.bone_ids, "\n\tThe BoneIds are dissimilar\n");
//		assert_eq!(non_ffi_mesh.bone_weights, conv_ffi.bone_weights, "\n\tThe BoneWeights are dissimilar\n");
//		assert_eq!(non_ffi_mesh.vbo_id, conv_ffi.vbo_id, "\n\tThe VBO IDs are dissimilar\n");
//
//
//		raylib_ffi::CloseWindow();
//
//		assert!(true);
//	}
//}