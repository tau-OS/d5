use smithay::{
    desktop::WindowSurfaceType,
    input::pointer::PointerHandle,
    utils::{Logical, Point},
};
use wayland_server::protocol::wl_surface::WlSurface;

use crate::state::GyakuState;

// See https://github.com/Smithay/smithay/blob/e9bdcb982f9c242dfe7d1c3629be6c0a18a4a1ee/smallvil/src/state.rs#L143
// Not sure if this belonds in the input utils.. maybe as an extension trait?
pub fn surface_under_pointer(
    state: &GyakuState,
    pointer: &PointerHandle<GyakuState>,
) -> Option<(WlSurface, Point<i32, Logical>)> {
    let pos = pointer.current_location();
    state
        .space
        .element_under(pos)
        .and_then(|(window, location)| {
            window
                .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                .map(|(s, p)| (s, p + location))
        })
}