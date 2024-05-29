
use crate::structures::{matrix::*, vectors::*, material::*};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
	pub vertex_count:	i32,
	pub triangle_count: i32,
	pub vertices:		*mut f32,
	pub texcoords:		*mut f32,
	pub texcoords2:		*mut f32,
	pub normals:		*mut f32,
	pub tangents:		*mut f32,
	pub colors:			*mut u8,
	pub indices:		*mut u16,
	pub anim_vertices:	*mut f32,
	pub anim_normals:	*mut f32,
	pub bone_ids:		*mut u8,
	pub bone_weights:	*mut f32,
	pub vao_id:			u32,
	pub vbo_id:			*mut u32,
}

impl Mesh {
	
	/// #### unload
	/// Wrapper for Raylib::UnloadMesh(mesh: Mesh).
	pub fn unload(&mut self) -> &mut Self {
		unsafe{ UnloadMesh(*self) }
		
		self
	}
	
	/// #### draw
	/// Wrapper for Raylib::DrawMesh(mesh: Mesh, material: Material, transform: Matrix).
	pub fn draw(&mut self, material: Material, transform: Matrix) -> &mut Self {
		unsafe{ DrawMesh(*self, material, transform) }
		
		self
	}
	/// #### draw_instanced
	/// Draw multiple instances of the same mesh.
	/// TODO: I don't know whay this doesn't work...
	pub fn draw_instanced(&mut self, material: Material, transforms: &[Matrix]) -> &mut Self {
		unsafe{ DrawMeshInstanced(*self, material, transforms.as_ptr(), transforms.len() as i32) }
		
		self
	}
	
	/// #### gen_cube
	/// Wrapper for Raylib::
	pub fn gen_cube(width: f32, height: f32, length: f32) -> Self {
		unsafe{ GenMeshCube(width, height, length) }
	}
	
}

//= Mesh management functions
extern "C" { fn UnloadMesh(mesh: Mesh); }
extern "C" { fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix); }
extern "C" { fn DrawMeshInstanced(mesh: Mesh, material: Material, transforms: *const Matrix, instances: i32); }

//= Mesh generation functions
extern "C" { fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh; }


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Model {
	pub transform:		Matrix,
	pub mesh_count:		i32,
	pub material_count:	i32,
	pub meshes:			*mut Mesh,
	pub materials:		*mut Material,
	pub mesh_material:	*mut i32,
	pub bone_count:		i32,
	pub bones:			*mut BoneInfo,
	pub bind_pose:		*mut Transform,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoneInfo {
	pub name:	[i8; 32],
	pub parent:	i32,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
	pub translation:	Vector3,
	pub rotation:		Quaternion,
	pub scale:			Vector3,
}