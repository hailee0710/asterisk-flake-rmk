#![no_main]
#![no_std]

mod pointing_processor;

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {
    #[register_processor(event)]
    fn pointing_processor_controller() -> crate::pointing_processor::PointingProcessorController {
        crate::pointing_processor::PointingProcessorController::new()
    }
}
