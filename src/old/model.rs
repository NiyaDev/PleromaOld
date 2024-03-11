

//= Imports
use super::{color::Color, material::Material, matrix::Matrix, vector::Vector3, mesh::*};


//= Structure and Enumerations

/// Model stored on GPU
#[derive(Debug)]
pub struct Model {
	pub transform:		Matrix,

	pub meshes:			Vec<Mesh>,

	pub materials:		Vec<Material>,
	pub mesh_material:	Vec<i32>,

	pub bone_count:		i32,
	pub bones:			*mut raylib_ffi::BoneInfo,
	pub bind_pose:		*mut raylib_ffi::Transform,
}
impl PartialEq for Model {
	fn eq(&self, other: &Self) -> bool {
		self.transform == other.transform &&
			self.meshes == other.meshes &&
			self.materials == other.materials &&
			self.mesh_material == other.mesh_material &&
			self.bone_count == other.bone_count &&
			self.bones == other.bones &&
			self.bind_pose == other.bind_pose
	}
}

/// Info on single bone
pub struct BoneInfo {
	pub name:		[i8; 32],
    pub parent:		i32,
}


//= Procedures

impl Model {
	
	//= Generation
	/// Wrapper for LoadModel
	pub fn load(filename: &str) -> Self {
		unsafe {return Model::from_ffi(raylib_ffi::LoadModel(raylib_ffi::rl_str!(filename))) }
	}
	/// Wrapper for LoadModelFromMesh
	pub fn load_mesh(mesh: Mesh) -> Self {
		unsafe { return Model::from_ffi(raylib_ffi::LoadModelFromMesh(mesh.into_ffi())) }
	}

	//= Drawing
	/// Warpper for DrawModel
	pub fn draw(&self, position: Vector3, scale: f32, tint: Color) {
		unsafe { raylib_ffi::DrawModel(self.into_ffi(), position.into_ffi(), scale, tint.into_ffi()) }
	}

	//= Conversion
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Model) -> Self {
		//* Grabbing Meshes */
		let mut mesh_vec = Vec::new();
		for i in 0..value.meshCount {
			unsafe { mesh_vec.push(Mesh::from_ffi(value.meshes.wrapping_offset(i as isize).as_mut().unwrap().to_owned())) }
		}

		//* Grabbing materials */
		let mut mat_vec = Vec::new();
		let mut mat_mesh_vec = Vec::new();
		for i in 0..value.materialCount {
			unsafe {
				mat_vec.push(Material::from_ffi(value.materials.wrapping_offset(i as isize).as_mut().unwrap().to_owned()));
				mat_mesh_vec.push(value.meshMaterial.wrapping_offset(i as isize).as_mut().unwrap().to_owned());
			}
		}

		Self {
			transform:		Matrix::from_ffi(value.transform),
			meshes:			mesh_vec,

			materials:		mat_vec,
			mesh_material:	mat_mesh_vec,

			bone_count:		value.boneCount,
			bones:			value.bones,

			bind_pose:		value.bindPose,
		}
	}
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Model {
		//* Grabbing Meshes */
		let mut ffi_meshes = Vec::new();
		for i in 0..self.meshes.len() {
			ffi_meshes.push(self.meshes.get(i).unwrap().into_ffi());
		}

		//* Grabbing Materials */
		let mut ffi_materials = Vec::new();
		let mut mesh_mat = Vec::new();
		for i in 0..self.materials.len() {
			ffi_materials.push(self.materials.get(i).unwrap().into_ffi());
			mesh_mat.push(*self.mesh_material.get(i).unwrap())
		}

		raylib_ffi::Model {
			transform:		self.transform.into_ffi(),

			meshCount:		self.meshes.len() as i32,
			meshes:			ffi_meshes.as_mut_slice().as_mut_ptr(),

			materialCount:	self.materials.len() as i32,
			materials:		ffi_materials.as_mut_slice().as_mut_ptr(),
			meshMaterial:	mesh_mat.as_mut_slice().as_mut_ptr(),

			boneCount:		self.bone_count,
			bones:			self.bones,

			bindPose:		self.bind_pose,
		}
	}

}