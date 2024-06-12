
use crate::{
	rl_str,
	bounds::*,
	color::*,
	image::*,
	material::*,
	matrix::*,
	vectors::*,
	
};


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
	/// #### export
	/// Wrapper for Raylib::ExportMesh(mesh: Mesh, filenmae: *const i8).
	pub fn export(&mut self, filename: &str) -> bool {
		unsafe{ ExportMesh(*self, rl_str!(filename)) }
	}
	
	/// #### get_bounds
	/// Wrapper for Raylib::GetMeshBoundingBox(mesh: Mesh).
	pub fn get_bounds(&mut self) -> BoundingBox {
		unsafe{ GetMeshBoundingBox(*self) }
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
	
	/// #### gen_poly
	/// Wrapper for Raylib::GenMeshPoly(width: f32, height: f32, length: f32).
	pub fn gen_poly(sides: i32, radius: f32) -> Self {
		unsafe{ GenMeshPoly(sides, radius) }
	}
	/// #### gen_plane
	/// Wrapper for Raylib::GenMeshPlane(width: f32, height: f32, length: f32).
	pub fn gen_plane(width: f32, length: f32, resx: i32, resz: i32) -> Self {
		unsafe{ GenMeshPlane(width, length, resx, resz) }
	}
	/// #### gen_cube
	/// Wrapper for Raylib::GenMeshCube(width: f32, height: f32, length: f32).
	pub fn gen_cube(width: f32, height: f32, length: f32) -> Self {
		unsafe{ GenMeshCube(width, height, length) }
	}
	/// #### gen_sphere
	/// Wrapper for Raylib::GenMeshSphere(radius: f32, rings: i32, slices: i32).
	pub fn gen_sphere(radius: f32, rings: i32, slices: i32) -> Self {
		unsafe{ GenMeshSphere(radius, rings, slices) }
	}
	/// #### gen_hemi_sphere
	/// Wrapper for Raylib::GenMesh(radius: f32, rings: i32, slices: i32).
	pub fn gen_hemi_sphere(radius: f32, rings: i32, slices: i32) -> Self {
		unsafe{ GenMeshHemiSphere(radius, rings, slices) }
	}
	/// #### gen_cylinder
	/// Wrapper for Raylib::GenMeshCylinder(radius: f32, rings: i32, slices: i32).
	pub fn gen_cylinder(radius: f32, height: f32, slices: i32) -> Self {
		unsafe{ GenMeshCylinder(radius, height, slices) }
	}
	/// #### gen_cone
	/// Wrapper for Raylib::GenMeshCone(radius: f32, rings: i32, slices: i32).
	pub fn gen_cone(radius: f32, height: f32, slices: i32) -> Self {
		unsafe{ GenMeshCone(radius, height, slices) }
	}
	/// #### gen_torus
	/// Wrapper for Raylib::GenMeshTorus(radius: f32, size: f32, radseg: i32, sides: i32).
	pub fn gen_torus(radius: f32, size: f32, radseg: i32, sides: i32) -> Self {
		unsafe{ GenMeshTorus(radius, size, radseg, sides) }
	}
	/// #### gen_knot
	/// Wrapper for Raylib::GenMeshKnot(radius: f32, size: f32, radseg: i32, sides: i32).
	pub fn gen_knot(radius: f32, size: f32, radseg: i32, sides: i32) -> Self {
		unsafe{ GenMeshKnot(radius, size, radseg, sides) }
	}
	/// #### gen_heightmap
	/// Wrapper for Raylib::GenMeshHeightmap(heightmap: ImageRl, size: Vector3).
	pub fn gen_heightmap(heightmap: Image, size: Vector3) -> Self {
		unsafe{ GenMeshHeightmap(heightmap.0, size) }
	}
	/// #### gen_cubicmap
	/// Wrapper for Raylib::GenMeshCubicmap(cubicmap: ImageRl, size: Vector3).
	pub fn gen_cubicmap(cubicmap: Image, size: Vector3) -> Self {
		unsafe{ GenMeshCubicmap(cubicmap.0, size) }
	}
	
	/// #### model
	/// Wrapper for Raylib::LoadModelFromMesh(mesh: Mesh) -> Model.
	pub fn model(&mut self) -> ModelRl {
		unsafe{ LoadModelFromMesh(*self) }
	}
	
}

//= Model management functions
extern "C" { fn LoadModelFromMesh(mesh: Mesh) -> ModelRl; }

//= Mesh management functions
extern "C" { fn UnloadMesh(mesh: Mesh); }
extern "C" { fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix); }
extern "C" { fn DrawMeshInstanced(mesh: Mesh, material: Material, transforms: *const Matrix, instances: i32); }
extern "C" { fn ExportMesh(mesh: Mesh, filenmae: *const i8) -> bool; }
extern "C" { fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox; }

//= Mesh generation functions
extern "C" { fn GenMeshPoly(sides: i32, radius: f32) -> Mesh; }
extern "C" { fn GenMeshPlane(width: f32, length: f32, resx: i32, resz: i32) -> Mesh; }
extern "C" { fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh; }
extern "C" { fn GenMeshSphere(radius: f32, rings: i32, slices: i32) -> Mesh; }
extern "C" { fn GenMeshHemiSphere(radius: f32, rings: i32, slices: i32) -> Mesh; }
extern "C" { fn GenMeshCylinder(radius: f32, height: f32, slices: i32) -> Mesh; }
extern "C" { fn GenMeshCone(radius: f32, height: f32, slices: i32) -> Mesh; }
extern "C" { fn GenMeshTorus(radius: f32, size: f32, radseg: i32, sides: i32) -> Mesh; }
extern "C" { fn GenMeshKnot(radius: f32, size: f32, radseg: i32, sides: i32) -> Mesh; }
extern "C" { fn GenMeshHeightmap(heightmap: ImageRl, size: Vector3) -> Mesh; }
extern "C" { fn GenMeshCubicmap(cubicmap: ImageRl, size: Vector3) -> Mesh; }


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelRl {
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
impl ModelRl {
	
	/// #### load
	/// Wrapper for Raylib::LoadModel(filename: *cont i8).
	pub fn load(filename: &str) -> Self {
		unsafe{ LoadModel(rl_str!(filename)) }
	}
	/// #### ready
	/// Wrapper for Raylib::LoadModel(model: Model) -> bool.
	pub fn ready(&mut self) -> bool {
		unsafe{ IsModelReady(*self) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadModel(model: Model).
	pub fn unload(&mut self) {
		unsafe{ UnloadModel(*self) }
	}
	
	/// #### set_material
	/// Wrapper for Raylib::SetModelMeshMaterial(model: *mut Model, mesh_id: i32, material_id: i32).
	pub fn set_material(&mut self, mesh_id: i32, material_id: i32) -> &mut Self {
		unsafe{ SetModelMeshMaterial(self, mesh_id, material_id) }
		
		self
	}
	
	/// #### draw
	/// Wrapper for Raylib::DrawModel(model: Model, position: Vector3, scale: f32, tint: Color) .
	pub fn draw(&mut self, position: Vector3, scale: f32, tint: Color) -> &mut Self {
		unsafe{ DrawModel(*self, position, scale, tint) }
		
		self
	}
	/// #### draw_ex
	/// Wrapper for Raylib::DrawModel(model: Model, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color) .
	pub fn draw_ex(&mut self, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color) -> &mut Self {
		unsafe{ DrawModelEx(*self, position, rotation_axis, rotation_angle, scale, tint) }
		
		self
	}
	/// #### draw_wires
	/// Wrapper for Raylib::DrawModel(model: Model, position: Vector3, scale: f32, tint: Color) .
	pub fn draw_wires(&mut self, position: Vector3, scale: f32, tint: Color) -> &mut Self {
		unsafe{ DrawModelWires(*self, position, scale, tint) }
		
		self
	}
	/// #### draw_wires_ex
	/// Wrapper for Raylib::DrawModel(model: Model, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color) .
	pub fn draw_wires_ex(&mut self, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color) -> &mut Self {
		unsafe{ DrawModelWiresEx(*self, position, rotation_axis, rotation_angle, scale, tint) }
		
		self
	}
	
	/// #### get_bounds
	/// Wrapper for Raylib::GetModelBoundingBox(model: Model) -> BoundingBox.
	pub fn get_bounds(&mut self) -> BoundingBox {
		unsafe{ GetModelBoundingBox(*self) }
	}
	
}

//= Model management functions
extern "C" { fn LoadModel(filename: *const i8) -> ModelRl; }
extern "C" { fn IsModelReady(model: ModelRl) -> bool; }
extern "C" { fn UnloadModel(model: ModelRl); }
extern "C" { fn GetModelBoundingBox(model: ModelRl) -> BoundingBox; }

//= Model drawing functions
extern "C" { fn DrawModel(model: ModelRl, position: Vector3, scale: f32, tint: Color); }
extern "C" { fn DrawModelEx(model: ModelRl, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color); }
extern "C" { fn DrawModelWires(model: ModelRl, position: Vector3, scale: f32, tint: Color); }
extern "C" { fn DrawModelWiresEx(model: ModelRl, position: Vector3, rotation_axis: Vector3, rotation_angle: f32, scale: Vector3, tint: Color); }

//= Material loading/unloading functions
extern "C" { fn SetModelMeshMaterial(model: *mut ModelRl, meshId: i32, materialId: i32); }


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelAnimation {
	pub bone_count: i32,
	pub frame_count: i32,
	pub bones: *mut BoneInfo,
	pub frame_poses: *mut *mut Transform,
	pub name: [u8;32],
}
impl ModelAnimation {
	
	/// #### load
	/// Wrapper for Raylib::LoadModelAnimations(filename: *const i8, anim_count: *mut i32) -> *mut ModelAnimations
	pub fn load(filename: &str) -> Vec<Self> {
		unsafe{
			let mut count = [0;1];
			let anims = LoadModelAnimations(rl_str!(filename), count.as_mut_ptr());
			let mut result = Vec::new();
			
			for i in 0..count[0] {
				result.push(anims.wrapping_add(i as usize).read())
			}
			
			result
		}
	}
	///// #### unload
	///// Wrapper for Raylib::UnloadModelAnimations(animations: *mut ModelAnimations, anim_count: i32);
	//pub fn unload(animations: Vec<Self>) {}
	///// #### valid
	///// Wrapper for Raylib::IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool.
	//pub fn valid(&mut self) -> bool {}
	//
	/// #### update
	/// Wrapper for Raylib::UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: i32);
	pub fn update(&mut self, model: ModelRl, frame: i32) -> &mut Self {
		unsafe{
			UpdateModelAnimation(model, *self, frame);
		}
		
		self
	}
	
}


//= Model animations loading/unloading functions
extern "C" { fn LoadModelAnimations(filename: *const i8, animCount: *mut i32) -> *mut ModelAnimation; }
extern "C" { fn UpdateModelAnimation(model: ModelRl, anim: ModelAnimation, frame: i32); }


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