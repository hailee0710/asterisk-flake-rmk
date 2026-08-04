use rmk::{
    event::{LayerChangeEvent, PointingProcessorEvent, publish_event},
    input_device::pointing::{CursorConfig, PointingMode, ScrollConfig},
};
use rmk::macros::processor;

#[processor(subscribe = [LayerChangeEvent])]
#[derive(Default)]
pub struct PointingProcessorController;

impl PointingProcessorController {
    pub fn new() -> Self {
        Self
    }

    async fn on_layer_change_event(&mut self, event: LayerChangeEvent) {
        match event.0 {
            2 => publish_event(PointingProcessorEvent {
                device_id: 255,
                mode: PointingMode::Scroll(ScrollConfig {
                    multiplier_x: 1,
                    divisor_x: 8,
                    multiplier_y: 1,
                    divisor_y: 8,
                    invert_x: false,
                    invert_y: false,
                }),
            }),
            _ => publish_event(PointingProcessorEvent {
                device_id: 255,
                mode: PointingMode::Cursor(CursorConfig {
                    multiplier_x: 1,
                    multiplier_y: 1,
                    invert_x: false,
                    invert_y: false,
                }),
            }),
        }
    }
}
