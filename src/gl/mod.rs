

//= Imports


//= Constants


//= Structures & Enumerations
#[derive(PartialEq)]
pub enum GlVersion {
	RlOpengl11 = 1,	// OpenGL 1.1
	RlOpengl21,		// OpenGL 2.1 (GLSL 120)
	RlOpengl33,		// OpenGL 3.3 (GLSL 330)
	RlOpengl43,		// OpenGL 4.3 (using GLSL 330)
	RlOpenglEs20,	// OpenGL ES 2.0 (GLSL 100)
	RlOpenglEs30,	// OpenGL ES 3.0 (GLSL 300 es)
}


//= Procedures

/// Load OpenGL extensions
// NOTE: External loader function must be provided
pub fn rl_load_extensions() {
	let version = rl_get_version();

	if version == GlVersion::RlOpengl33 {
		
	}

}

pub fn rl_get_version() -> GlVersion {
	GlVersion::RlOpengl43
}