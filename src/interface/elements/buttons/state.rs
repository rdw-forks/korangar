use derive_new::new;
use num::Zero;

use crate::input::UserEvent;
use crate::interface::traits::Element;
use crate::interface::types::*;
use crate::graphics::Renderer;

#[derive(new)]
pub struct StateButton {
    text: &'static str,
    event: UserEvent,
    selector: Box<dyn Fn(&StateProvider) -> bool>,
    #[new(value = "Size::zero()")]
    cached_size: Size,
    #[new(value = "Position::zero()")]
    cached_position: Position,
}

impl Element for StateButton {

    fn resolve(&mut self, placement_resolver: &mut PlacementResolver, _interface_settings: &InterfaceSettings, theme: &Theme) {
        let (size, position) = placement_resolver.allocate(&theme.button.size_constraint);
        self.cached_size = size.finalize();
        self.cached_position = position;
    }

    fn hovered_element(&self, mouse_position: Position) -> HoverInformation {
        let absolute_position = mouse_position - self.cached_position;

        if absolute_position.x >= 0.0 && absolute_position.y >= 0.0 && absolute_position.x <= self.cached_size.x && absolute_position.y <= self.cached_size.y {
            return HoverInformation::Hovered;
        }

        HoverInformation::Missed
    }

    fn left_click(&mut self, _force_update: &mut bool) -> Option<ClickAction> {
        Some(ClickAction::Event(self.event.clone()))
    }

    fn render(&self, renderer: &mut Renderer, state_provider: &StateProvider, interface_settings: &InterfaceSettings, theme: &Theme, parent_position: Position, clip_size: Size, hovered_element: Option<&dyn Element>, _second_theme: bool) {
        let absolute_position = parent_position + self.cached_position;
        let clip_size = vector2!(f32::min(clip_size.x, absolute_position.x + self.cached_size.x), f32::min(clip_size.y, absolute_position.y + self.cached_size.y));

        match matches!(hovered_element, Some(reference) if std::ptr::eq(reference as *const _ as *const (), self as *const _ as *const ())) {
            true => renderer.render_rectangle(absolute_position, self.cached_size, clip_size, *theme.button.border_radius * *interface_settings.scaling, *theme.button.hovered_background_color),
            false => renderer.render_rectangle(absolute_position, self.cached_size, clip_size, *theme.button.border_radius * *interface_settings.scaling, *theme.button.background_color),
        }

        renderer.render_checkbox(absolute_position + *theme.button.icon_offset * *interface_settings.scaling, *theme.button.icon_size * *interface_settings.scaling, clip_size, *theme.button.foreground_color, (self.selector)(state_provider));
        renderer.render_text(self.text, absolute_position + *theme.button.icon_text_offset * *interface_settings.scaling, clip_size, *theme.button.foreground_color, *theme.button.font_size * *interface_settings.scaling);
    }
}
