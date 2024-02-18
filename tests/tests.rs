


//= Imports
use navia::mesh::*;


//= Tests

//* Mesh */
#[test]
fn mesh_conversion() {
	unsafe {
		raylib_ffi::SetTraceLogLevel(raylib_ffi::enums::TraceLogLevel::None as i32);
		raylib_ffi::InitWindow(1280, 720, raylib_ffi::rl_str!("mesh_conversion"));

		let non_ffi_mesh = Mesh::gen_cube(1.0,1.0,1.0);
		//let ffi_mesh = raylib_ffi::GenMeshCube(1.0, 1.0, 1.0);
		let conv_ffi = Mesh::from_ffi(non_ffi_mesh.into_ffi());

		assert_eq!(non_ffi_mesh.vertices, conv_ffi.vertices, "\n\tThe Vertices are dissimilar\n");
		assert_eq!(non_ffi_mesh.texcoords, conv_ffi.texcoords, "\n\tThe Texcoords are dissimilar\n");
		assert_eq!(non_ffi_mesh.texcoords2, conv_ffi.texcoords2, "\n\tThe Texcoord2s are dissimilar\n");
		assert_eq!(non_ffi_mesh.normals, conv_ffi.normals, "\n\tThe Normals are dissimilar\n");
		assert_eq!(non_ffi_mesh.tangents, conv_ffi.tangents, "\n\tThe Tangents are dissimilar\n");
		assert_eq!(non_ffi_mesh.colors, conv_ffi.colors, "\n\tThe Colors are dissimilar\n");
		assert_eq!(non_ffi_mesh.indices, conv_ffi.indices, "\n\tThe Indices are dissimilar\n");
		assert_eq!(non_ffi_mesh.anim_vertices, conv_ffi.anim_vertices, "\n\tThe AnimVertices are dissimilar\n");
		assert_eq!(non_ffi_mesh.anim_normals, conv_ffi.anim_normals, "\n\tThe AnimNormals are dissimilar\n");
		assert_eq!(non_ffi_mesh.bone_ids, conv_ffi.bone_ids, "\n\tThe BoneIds are dissimilar\n");
		assert_eq!(non_ffi_mesh.bone_weights, conv_ffi.bone_weights, "\n\tThe BoneWeights are dissimilar\n");
		assert_eq!(non_ffi_mesh.vbo_id, conv_ffi.vbo_id, "\n\tThe VBO IDs are dissimilar\n");


		raylib_ffi::CloseWindow();

		assert!(true);
	}
}

//#[test]
//fn testing_mesh_conversion() {
//	unsafe {
//		//raylib_ffi::SetTraceLogLevel(raylib_ffi::enums::TraceLogLevel::None as i32);
//		//raylib_ffi::InitWindow(1280, 720, raylib_ffi::rl_str!("testing_mesh_conversion"));
//
//		let non_ffi_mesh = Mesh::gen_cube(1.0, 1.0, 1.0);
//		let ffi_mesh = raylib_ffi::GenMeshCube(1.0, 1.0, 1.0);
//		let conv_ffi = Mesh::from_ffi(ffi_mesh);
//
//		assert_eq!(non_ffi_mesh, conv_ffi);
//
//		//raylib_ffi::CloseWindow();
//	}
//}

//* Model */
//#[test]
//fn testing_model_conversion() {
//	let non_ffi_mesh = Mesh::gen_cube(1.0, 1.0, 1.0);
//	let non_ffi_model = Model::load_mesh(non_ffi_mesh);
//	let ffi_mesh = unsafe{raylib_ffi::GenMeshCube(1.0, 1.0, 1.0)};
//	let ffi_model = unsafe { raylib_ffi::LoadModelFromMesh(ffi_mesh) };
//
//	assert_eq!(non_ffi_model, Model::from_ffi(ffi_model))
//}