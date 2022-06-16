use anyhow::Result;
use filesystem::*;
use graphics::*;
use winit::{dpi::PhysicalSize, event::WindowEvent};
#[allow(dead_code)]
pub struct App {
    graphics: GraphicSystem,
    assets: FileSystem,
    config: Option<()>,
    pub window_size: PhysicalSize<u32>,
}

impl App {
    pub async fn new(window_ref: &winit::window::Window) -> Result<App> {
        let graphics = GraphicSystem::new(window_ref).await;
        let assets = FileSystem::new("/assets", "./resources")?;
        Ok(Self {
            graphics,
            assets,
            config: Some(()),
            window_size: window_ref.inner_size(),
        })
    }

    pub fn load_assets() -> Result<()> {
        todo!()
    }

    pub fn handle_input(&mut self, _event: &WindowEvent) -> Result<()> {
        todo!()
    }

    pub fn update(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }

    pub fn handle_window_resize(&mut self, _new_size: winit::dpi::PhysicalSize<u32>) {}
}
