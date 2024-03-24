
extern "C" { pub fn IsKeyPressed(key: i32) -> bool; }
extern "C" { pub fn IsKeyDown(key: i32) -> bool; }
extern "C" { pub fn IsKeyReleased(key: i32) -> bool; }
extern "C" { pub fn IsKeyUp(key: i32) -> bool; }
extern "C" { pub fn IsMouseButtonPressed(button: i32) -> bool; }
extern "C" { pub fn IsMouseButtonDown(button: i32) -> bool; }
extern "C" { pub fn IsMouseButtonReleased(button: i32) -> bool; }
extern "C" { pub fn IsMouseButtonUp(button: i32) -> bool; }
extern "C" { pub fn GetMouseX() -> f32; }
extern "C" { pub fn GetMouseY() -> f32; }
extern "C" { pub fn IsGamepadButtonPressed(gamepad: i32, button: i32) -> bool; }
extern "C" { pub fn IsGamepadButtonDown(gamepad: i32, button: i32) -> bool; }
extern "C" { pub fn IsGamepadButtonReleased(gamepad: i32, button: i32) -> bool; }
extern "C" { pub fn IsGamepadButtonUp(gamepad: i32, button: i32) -> bool; }
extern "C" { pub fn GetGamepadAxisMovement(gamepad: i32, axis: i32) -> f32; }