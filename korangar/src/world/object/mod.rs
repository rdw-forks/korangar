use std::sync::Arc;

use cgmath::Matrix4;
use derive_new::new;
use korangar_interface::elements::PrototypeElement;
use korangar_interface::windows::PrototypeWindow;
use ragnarok_formats::transform::Transform;
use ragnarok_packets::ClientTick;
use wgpu::RenderPass;

#[cfg(feature = "debug")]
use super::MarkerIdentifier;
use super::Model;
#[cfg(feature = "debug")]
use crate::graphics::{Color, DeferredRenderer, MarkerRenderer};
use crate::{Camera, GeometryRenderer, Renderer};

#[derive(PrototypeElement, PrototypeWindow, new)]
pub struct Object {
    pub name: Option<String>,
    pub model_name: String,
    pub model: Arc<Model>,
    pub transform: Transform,
}

impl Object {
    pub fn render_geometry<T>(
        &self,
        render_target: &mut T::Target,
        render_pass: &mut RenderPass,
        renderer: &T,
        camera: &dyn Camera,
        client_tick: ClientTick,
        time: f32,
    ) where
        T: Renderer + GeometryRenderer,
    {
        self.model
            .render_geometry(render_target, render_pass, renderer, camera, &self.transform, client_tick, time);
    }

    pub fn get_bounding_box_matrix(&self) -> Matrix4<f32> {
        self.model.get_bounding_box_matrix(&self.transform)
    }

    #[cfg(feature = "debug")]
    pub fn render_bounding_box(
        &self,
        render_target: &mut <DeferredRenderer as Renderer>::Target,
        render_pass: &mut RenderPass,
        renderer: &DeferredRenderer,
        camera: &dyn Camera,
        color: Color,
    ) {
        self.model
            .render_bounding_box(render_target, render_pass, renderer, camera, &self.transform, color);
    }

    #[cfg(feature = "debug")]
    pub fn render_marker<T>(
        &self,
        render_target: &mut T::Target,
        render_pass: &mut RenderPass,
        renderer: &T,
        camera: &dyn Camera,
        marker_identifier: MarkerIdentifier,
        hovered: bool,
    ) where
        T: Renderer + MarkerRenderer,
    {
        renderer.render_marker(
            render_target,
            render_pass,
            camera,
            marker_identifier,
            self.transform.position,
            hovered,
        );
    }
}
