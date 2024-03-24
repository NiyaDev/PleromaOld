

/// Shader wrapper
pub struct Shader {
	pub shader: ShaderRl,
	pub locations: Vec<i32>,
}

#[repr(C)]
pub struct ShaderRl {
	pub id: u32,
    pub locs: *mut i32,
}
