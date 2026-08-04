#![no_main]
#![no_std]

use rmk::input_device::pointing::PointingProcessorController;
use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {
    use super::*;

    #[register_processor(event)]
    fn pointing_processor_controller() -> PointingProcessorController {
        PointingProcessorController::new()
    }
}
