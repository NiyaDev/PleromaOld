


//= Imports
use std::mem;
use crate::{color::Color, vector::*};


//= Structures and Enumeration

/// Mesh stored in RAM
#[derive(Debug, PartialEq)]
pub struct Mesh {
	pub vao_id:			u32,

	pub triangle_count:	i32,

	pub vertices:		Vec<Vector3>,

	pub texcoords:		Vec<Vector2>,
	pub texcoords2:		Vec<Vector2>,

	pub normals:		Vec<Vector3>,

	pub tangents:		Vec<f32>,

	pub colors:			Vec<Color>,

	pub indices:		Vec<u16>,
	
	pub vbo_id:			Vec<u32>,

	// TODO:
	pub anim_vertices:	*mut f32,
	pub anim_normals:	*mut f32,

	pub bone_ids:		*mut u8,
	pub bone_weights:	*mut f32,
		
	//pub vbo_id:			*mut u32,
}

//= Procedures

impl Mesh {
	
	//= Generation
	/// Wrapper for GenMeshCube
	pub fn gen_cube(width: f32, height: f32, length: f32) -> Self {
		unsafe { return Mesh::from_ffi(raylib_ffi::GenMeshCube(width, height, length)) }
	}

	//= Conversion
	/// Converting from FFI version
	// TODO
	pub fn from_ffi(value: raylib_ffi::Mesh) -> Self {
		//* Grabbing Vertices */
		let mut vert_vec = Vec::new();
		let mut offset = 0;
		if value.vertices != std::ptr::null_mut() {
			for _i in 0..value.vertexCount {
				unsafe {
					let vect = Vector3{
						x: value.vertices.wrapping_offset(offset as isize).as_mut().unwrap().to_owned(),
						y: value.vertices.wrapping_offset(offset as isize + 1).as_mut().unwrap().to_owned(),
						z: value.vertices.wrapping_offset(offset as isize + 2).as_mut().unwrap().to_owned(),
					};
					offset += 3;
					vert_vec.push(vect);
				}
			}
		}

		//* Grabbing Texcoords */
		let mut texcoords_vec = Vec::new();
		offset = 0;
		if value.texcoords != std::ptr::null_mut() {
			for _i in 0..value.vertexCount {
				unsafe {
					let vect = Vector2{
						x: value.texcoords.wrapping_offset(offset as isize).as_mut().unwrap().to_owned(),
						y: value.texcoords.wrapping_offset(offset as isize + 1).as_mut().unwrap().to_owned(),
					};
					offset += 2;
					texcoords_vec.push(vect);
				}
			}
		}
		//* Grabbing Texcoords 2 */
		let mut texcoords2_vec = Vec::new();
		if value.texcoords2 != std::ptr::null_mut() {
			offset = 0;
			for _i in 0..value.vertexCount {
				unsafe {
					let vect = Vector2{
						x: value.texcoords.wrapping_offset(offset as isize).as_mut().unwrap().to_owned(),
						y: value.texcoords.wrapping_offset(offset as isize + 1).as_mut().unwrap().to_owned(),
					};
					offset += 2;
					texcoords2_vec.push(vect);
				}
			}
		}

		//* Grabbing Normals */
		let mut normals_vec = Vec::new();
		offset = 0;
		if value.normals != std::ptr::null_mut() {
			for _i in 0..value.vertexCount {
				unsafe {
					let vect = Vector3{
						x: value.normals.wrapping_offset(offset as isize).as_mut().unwrap().to_owned(),
						y: value.normals.wrapping_offset(offset as isize + 1).as_mut().unwrap().to_owned(),
						z: value.normals.wrapping_offset(offset as isize + 2).as_mut().unwrap().to_owned(),
					};
					offset += 3;
					normals_vec.push(vect);
				}
			}
		}

		//* Grabbing Tangents */
		let mut tangents_vec = Vec::new();
		if value.tangents != std::ptr::null_mut() {
			for i in 0..value.vertexCount {
				unsafe { tangents_vec.push(value.tangents.wrapping_offset(i as isize).as_mut().unwrap().to_owned()); }
			}
		}

		//* Grabbing Colors */
		let mut colors_vec = Vec::new();
		offset = 0;
		if value.colors != std::ptr::null_mut() {
			for _i in 0..value.vertexCount {
				unsafe {
					let vect = Color{
						r: value.colors.wrapping_offset(offset as isize).as_mut().unwrap().to_owned(),
						g: value.colors.wrapping_offset(offset as isize + 1).as_mut().unwrap().to_owned(),
						b: value.colors.wrapping_offset(offset as isize + 2).as_mut().unwrap().to_owned(),
						a: value.colors.wrapping_offset(offset as isize + 3).as_mut().unwrap().to_owned(),
					};
					offset += 4;
					colors_vec.push(vect);
				}
			}
		}
		
		//* Grabbing Indices */
		let mut indices_vec = Vec::new();
		if value.indices != std::ptr::null_mut() {
			for i in 0..36 {
				unsafe { indices_vec.push(value.indices.wrapping_offset(i as isize).as_mut().unwrap().to_owned()); }
			}
		}

		//* Grabbing VBO IDs */
		let mut vbo_vec = Vec::new();
		if value.vboId != std::ptr::null_mut() {
			for i in 0..7 {
				unsafe { vbo_vec.push(value.vboId.wrapping_offset(i as isize).as_mut().unwrap().to_owned()); }
			}
		}
		

		Self {
			vao_id:			value.vaoId,

			vertices:		vert_vec,

			texcoords:		texcoords_vec,
			texcoords2:		texcoords2_vec,

			normals:		normals_vec,

			triangle_count:	value.triangleCount,

			tangents:		tangents_vec,
			colors:			colors_vec,
			indices:		indices_vec,

			vbo_id:			vbo_vec,

			anim_vertices:	value.animVertices,
			anim_normals:	value.animNormals,
			bone_ids:		value.boneIds,
			bone_weights:	value.boneWeights,
		}
	}
	/// Converting to FFI version
	// TODO
	pub fn into_ffi(&self) -> raylib_ffi::Mesh {
		//* Grabbing Vertices */
		let mut ffi_vertices = Vec::new();
		for i in 0..self.vertices.len() {
			ffi_vertices.push(self.vertices.get(i).unwrap().x);
			ffi_vertices.push(self.vertices.get(i).unwrap().y);
			ffi_vertices.push(self.vertices.get(i).unwrap().z);
		}
		let vertices = ffi_vertices.as_mut_slice().as_mut_ptr();

		//* Grabbing TexCoords */
		let mut ffi_texcoords = Vec::new();
		for i in 0..self.texcoords.len() {
			ffi_texcoords.push(self.texcoords.get(i).unwrap().x);
			ffi_texcoords.push(self.texcoords.get(i).unwrap().y);
		}
		let texcoords = ffi_texcoords.as_mut_slice().as_mut_ptr();
		//* Grabbing TexCoords 2 */
		let mut ffi_texcoords2 = Vec::new();
		for i in 0..self.texcoords2.len() {
			ffi_texcoords2.push(self.texcoords2.get(i).unwrap().x);
			ffi_texcoords2.push(self.texcoords2.get(i).unwrap().y);
		}
		let texcoords2 = ffi_texcoords2.as_mut_slice().as_mut_ptr();

		//* Grabbing Normals */
		let mut ffi_normals = Vec::new();
		for i in 0..self.normals.len() {
			ffi_normals.push(self.normals.get(i).unwrap().x);
			ffi_normals.push(self.normals.get(i).unwrap().y);
			ffi_normals.push(self.normals.get(i).unwrap().z);
		}
		let normals = ffi_normals.as_mut_slice().as_mut_ptr();

		//* Grabbing Tangents */
		let mut ffi_tangents = Vec::new();
		for i in 0..self.tangents.len() {
			ffi_tangents.push(*self.tangents.get(i).unwrap());
		}
		let tangents = ffi_tangents.as_mut_slice().as_mut_ptr();

		//* Grabbing Colors */
		let mut ffi_colors = Vec::new();
		for i in 0..self.colors.len() {
			ffi_colors.push(self.colors.get(i).unwrap().r);
			ffi_colors.push(self.colors.get(i).unwrap().g);
			ffi_colors.push(self.colors.get(i).unwrap().b);
			ffi_colors.push(self.colors.get(i).unwrap().a);
		}
		let colors = ffi_colors.as_mut_slice().as_mut_ptr();

		//* Grabbing Indices */
		let mut ffi_indices = Vec::new();
		for i in 0..self.indices.len() {
			ffi_indices.push(*self.indices.get(i).unwrap());
		}
		let indices = ffi_indices.as_mut_slice().as_mut_ptr();

		//* Grabbing VBO IDs */
		let mut ffi_vbo = Vec::new();
		for i in 0..self.vbo_id.len() {
			ffi_vbo.push(*self.vbo_id.get(i).unwrap());
		}
		let vbo_id = ffi_vbo.as_mut_slice().as_mut_ptr();

		let mesh = raylib_ffi::Mesh {
			vertexCount:	self.vertices.len() as i32,
			vertices:		vertices,
			triangleCount:	self.triangle_count,
			texcoords:		texcoords,
			texcoords2:		texcoords2,
			normals:		normals,
			tangents:		tangents,
			colors:			colors,
			indices:		indices,
			animVertices:	self.anim_vertices,
			animNormals:	self.anim_normals,
			boneIds:		self.bone_ids,
			boneWeights:	self.bone_weights,
			vaoId:			self.vao_id,
			vboId:			vbo_id,
		};
		//mem::forget(vertices);
		//mem::forget(texcoords);
		//mem::forget(texcoords2);
		//mem::forget(normals);
		//mem::forget(tangents);
		//mem::forget(colors);
		//mem::forget(indices);
		//mem::forget(vbo_id);

		mesh
	}

}