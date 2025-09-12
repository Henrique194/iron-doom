use super::{ScreenBuffer, SCREEN_HEIGHT, SCREEN_HEIGHT_4_3, SCREEN_WIDTH};
use anyhow::{Error, Result};
use bevy::prelude::*;
use exit::macros::sys_fail;
use game_state::conditions::in_playing_state;
use sdl2::{
    pixels::{Color, Palette, PixelFormatEnum},
    render::{CanvasBuilder, Texture, TextureAccess, WindowCanvas},
    surface::Surface,
    video::{Window, WindowBuilder},
    VideoSubsystem,
};

pub struct SdlWindowPlugin;

impl Plugin for SdlWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<SdlWindow>()
            .add_systems(Last, update_window.run_if(in_playing_state()));
    }
}

#[sys_fail]
fn update_window(mut sdl_window: NonSendMut<SdlWindow>, screen: Res<ScreenBuffer>) {
    sdl_window.clear_screen();
    sdl_window.update_paletted_surface(&screen);
    sdl_window.update_rgba_surface()?;
    sdl_window.update_texture()?;
    sdl_window.update_upscaled_texture()?;
    sdl_window.update_canvas()?;
    sdl_window.update_screen();
}

pub(crate) struct SdlWindow {
    paletted_surface: Surface<'static>,
    rgba_surface: Surface<'static>,
    rgba_texture: Texture,
    upscaled_texture: Texture,
    window_canvas: WindowCanvas,
}

impl SdlWindow {
    pub fn update_palette(&mut self, palette: &Palette) -> Result<()> {
        self.paletted_surface
            .set_palette(palette)
            .map_err(Error::msg)
    }

    /// Make sure the pillarboxes are kept clear each frame.
    fn clear_screen(&mut self) {
        self.window_canvas.clear();
    }

    fn update_paletted_surface(&mut self, buffer: &ScreenBuffer) {
        self.paletted_surface
            .with_lock_mut(|pixels| buffer.copy_to_slice(pixels));
    }

    /// Blit from the paletted 8-bit surface to the 32-bit RGBA
    /// surface that we can load into the RGBA texture.
    fn update_rgba_surface(&mut self) -> Result<()> {
        let dst = &mut self.rgba_surface;
        self.paletted_surface
            .blit(None, dst, None)
            .map_err(Error::msg)?;
        Ok(())
    }

    /// Update the RGBA texture with the contents of the RGBA surface.
    fn update_texture(&mut self) -> Result<()> {
        let pitch = self.rgba_surface.pitch();
        self.rgba_surface
            .with_lock(|pixels| self.rgba_texture.update(None, pixels, pitch as usize))
            .map_err(Error::msg)
    }

    /// Render RGBA texture into the upscaled texture using "nearest" integer scaling.
    fn update_upscaled_texture(&mut self) -> Result<()> {
        self.window_canvas
            .with_texture_canvas(&mut self.upscaled_texture, |canvas| {
                canvas.copy(&self.rgba_texture, None, None).unwrap()
            })
            .map_err(Error::msg)
    }

    /// Render upscaled texture to window canvas using linear scaling.
    fn update_canvas(&mut self) -> Result<()> {
        self.window_canvas
            .copy(&self.upscaled_texture, None, None)
            .map_err(Error::msg)
    }

    fn update_screen(&mut self) {
        self.window_canvas.present();
    }
}

impl FromWorld for SdlWindow {
    fn from_world(_world: &mut World) -> Self {
        let window_creator = SdlWindowCreator::try_new().unwrap();
        window_creator.create().unwrap()
    }
}

struct SdlWindowCreator {
    video_sys: VideoSubsystem,
}

impl SdlWindowCreator {
    pub fn try_new() -> Result<SdlWindowCreator> {
        let sdl_ctx = sdl2::init().map_err(Error::msg)?;
        let video_sys = sdl_ctx.video().map_err(Error::msg)?;
        Ok(SdlWindowCreator { video_sys })
    }

    pub fn create(&self) -> Result<SdlWindow> {
        let window = self.create_sdl_window()?;
        let window_canvas = self.create_window_canvas(window)?;
        let paletted_surface = self.create_paletted_surface()?;
        let rgba_surface = self.create_rgba_surface(&window_canvas)?;
        let rgba_texture = self.create_rgba_texture(&window_canvas)?;
        let upscaled_texture = self.create_upscaled_texture(&window_canvas)?;

        let mouse_util = self.video_sys.sdl().mouse();
        mouse_util.set_relative_mouse_mode(true);

        Ok(SdlWindow {
            paletted_surface,
            rgba_surface,
            rgba_texture,
            upscaled_texture,
            window_canvas,
        })
    }

    fn create_sdl_window(&self) -> Result<Window> {
        let mut window_builder = WindowBuilder::new(&self.video_sys, "", 0, 0);
        let (x, y) = self.get_window_position()?;
        window_builder
            .resizable()
            .allow_highdpi()
            .fullscreen_desktop()
            .position(x, y);
        let mut window = window_builder.build()?;
        window.set_minimum_size(SCREEN_WIDTH, SCREEN_HEIGHT_4_3)?;
        Ok(window)
    }

    fn get_window_position(&self) -> Result<(i32, i32)> {
        // Get center of the screen
        let bounds = self.video_sys.display_bounds(0).map_err(Error::msg)?;
        let w = bounds.width() / 2;
        let h = bounds.height() / 2;
        let x = bounds.x().saturating_add_unsigned(w);
        let y = bounds.y().saturating_add_unsigned(h);
        Ok((x, y))
    }

    fn create_window_canvas(&self, window: Window) -> Result<WindowCanvas> {
        let mut canvas = CanvasBuilder::new(window).target_texture();
        let display_mode = self.video_sys.current_display_mode(0).map_err(Error::msg)?;
        if display_mode.refresh_rate > 0 {
            canvas = canvas.present_vsync();
        }

        let mut canvas = canvas.build()?;

        // Important: Set the "logical size" of the rendering context. At the same
        // time this also defines the aspect ratio that is preserved while scaling
        // and stretching the texture into the window.
        canvas.set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT_4_3)?;

        // Disable integer scales for rendering.
        canvas.set_integer_scale(false).map_err(Error::msg)?;

        // Blank out the full screen area in case there is any junk in
        // the borders that won't otherwise be overwritten.
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Ok(canvas)
    }

    fn create_surface(&self, pixel_format: PixelFormatEnum) -> Result<Surface<'static>> {
        let pixel_masks = pixel_format.into_masks().map_err(Error::msg)?;
        let w = SCREEN_WIDTH;
        let h = SCREEN_HEIGHT;
        let mut surface = Surface::from_pixelmasks(w, h, &pixel_masks).map_err(Error::msg)?;
        surface.fill_rect(None, Color::BLACK).map_err(Error::msg)?;
        Ok(surface)
    }

    fn create_paletted_surface(&self) -> Result<Surface<'static>> {
        self.create_surface(PixelFormatEnum::Index8)
    }

    fn create_rgba_surface(&self, window_canvas: &WindowCanvas) -> Result<Surface<'static>> {
        let pixel_format = window_canvas.window().window_pixel_format();
        self.create_surface(pixel_format)
    }

    fn create_texture(
        &self,
        window_canvas: &WindowCanvas,
        access: TextureAccess,
        width: u32,
        height: u32,
    ) -> Result<Texture> {
        let texture_creator = window_canvas.texture_creator();
        let pixel_format = window_canvas.window().window_pixel_format();
        let texture = texture_creator.create_texture(pixel_format, access, width, height)?;
        Ok(texture)
    }

    fn create_rgba_texture(&self, window_canvas: &WindowCanvas) -> Result<Texture> {
        // Set the scaling quality for rendering the intermediate texture into
        // the upscaled texture to "nearest", which is gritty and pixelated and
        // resembles software scaling pretty well.
        sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "nearest");

        let access = TextureAccess::Streaming;
        let w = SCREEN_WIDTH;
        let h = SCREEN_HEIGHT;
        self.create_texture(window_canvas, access, w, h)
    }

    fn create_upscaled_texture(&self, window_canvas: &WindowCanvas) -> Result<Texture> {
        // Set the scaling quality for rendering the upscaled texture to "linear",
        // which looks much softer and smoother than "nearest" but does a better
        // job at downscaling from the upscaled texture to screen.
        sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "linear");

        let (w_upscale, h_upscale) = self.get_upscale_factors(window_canvas)?;
        let w = w_upscale * SCREEN_WIDTH;
        let h = h_upscale * SCREEN_HEIGHT;
        let access = TextureAccess::Target;
        self.create_texture(window_canvas, access, w, h)
    }

    fn get_upscale_factors(&self, window_canvas: &WindowCanvas) -> Result<(u32, u32)> {
        let (mut w, mut h) = window_canvas.output_size().map_err(Error::msg)?;

        // When the screen or window dimensions do not match the aspect ratio
        // of the texture, the rendered area is scaled down to fit. Calculate
        // the actual dimensions of the rendered area.
        if w * SCREEN_HEIGHT_4_3 < h * SCREEN_WIDTH {
            // Tall window.
            h = w * SCREEN_HEIGHT_4_3 / SCREEN_WIDTH;
        } else {
            // Wide window.
            w = h * SCREEN_WIDTH / SCREEN_HEIGHT_4_3;
        }

        // Pick texture size the next integer multiple of the screen dimensions.
        // If one screen dimension matches an integer multiple of the original
        // resolution, there is no need to overscale in this direction.
        let mut w_upscale = w.div_ceil(SCREEN_WIDTH);
        let mut h_upscale = h.div_ceil(SCREEN_HEIGHT);

        // Minimum texture dimensions of 320x200.
        if w_upscale < 1 {
            w_upscale = 1;
        }
        if h_upscale < 1 {
            h_upscale = 1;
        }

        Ok((w_upscale, h_upscale))
    }
}
