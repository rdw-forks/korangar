use procedural::*;

use crate::interface::{ Window, PrototypeWindow };
use crate::interface::InterfaceSettings;
use crate::interface::{ WindowCache, Size };
use crate::interface::FramedWindow;

#[derive(Default)]
pub struct ProfilerWindow {}

impl ProfilerWindow {

    pub const WINDOW_CLASS: &'static str = "profiler";
}

impl PrototypeWindow for ProfilerWindow {

    fn window_class(&self) -> Option<&str> {
        Self::WINDOW_CLASS.into()
    }

    fn to_window(&self, window_cache: &WindowCache, interface_settings: &InterfaceSettings, avalible_space: Size) -> Box<dyn Window + 'static> {

        let elements = vec![];

        Box::from(FramedWindow::new(window_cache, interface_settings, avalible_space, "Profiler".to_string(), Self::WINDOW_CLASS.to_string().into(), elements, constraint!(200 > 250 < 300, ?)))
    }
}
