use std::sync::Arc;

use crate::data::LOG_MODULE_NAME;

use super::RenderManager;
use super::frame_context::WgpuFrameContext;
use super::wgpu_context::WgpuContext;

use asn_gui_core::{TAsnGuiHandler, TAsnRenderManager};
use asn_logger::*;
use asn_winit::WinitWindow;

impl<H> TAsnRenderManager for RenderManager<H>
where
    H: TAsnGuiHandler<GraphContext = WgpuContext, FrameContext = WgpuFrameContext>,
{
    type Window = WinitWindow;

    fn init(&mut self, w: Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let context = match pollster::block_on(WgpuContext::new(w)) {
            Ok(context) => context,
            Err(e) => {
                m_error!("Failed to create GPU state: {e}");
                return Err(Box::new(std::io::Error::other(format!(
                    "RenderManager:init error: {e}"
                ))));
            }
        };

        {
            let mut h = match self.h.lock() {
                Ok(h) => h,
                Err(e) => {
                    return Err(Box::new(std::io::Error::other(format!(
                        "RenderManager:draw error - handler cant unlock - {e}"
                    ))));
                }
            };
            // h.init(&context)
        }

        self.s = Some(context);

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let s = match self.s.as_mut() {
            Some(s) => s,
            None => {
                return Err(Box::new(std::io::Error::other(format!(
                    "RenderManager:resize error - manager not initialized"
                ))));
            }
        };

        if let Err(e) = s.resize(width, height) {
            return Err(Box::new(std::io::Error::other(format!(
                "RenderManager:resize error - {e}"
            ))));
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // begin frame
        let _r = match self.s.as_mut() {
            Some(r) => r,
            None => {
                return Err(Box::new(std::io::Error::other(format!(
                    "RenderManager:draw error - manager not initialized"
                ))));
            }
        };

        // let fcx = match WgpuFrameContext::new(&r.surface, &r.device) {
        //     Ok(fcx) => fcx,
        //     Err(e) => {
        //         return Err(Box::new(std::io::Error::other(format!(
        //             "RenderManager:draw error - {e}"
        //         ))));
        //     }
        // };

        // {
        //     let mut h = match self.h.lock() {
        //         Ok(h) => h,
        //         Err(e) => {
        //             return Err(Box::new(std::io::Error::other(format!(
        //                 "RenderManager:draw error - handler cant unlock - {e}"
        //             ))));
        //         }
        //     };
        //     h.draw(&fcx)
        // }
        // end frame

        // let _frame_duration = fcx.frame_start.elapsed();

        // Update render statistics
        // self.render_stats.frame_count += 1;
        // self.render_stats.total_render_time += frame_duration;

        // Log performance every 60 frames
        // if self.render_stats.frame_count % 60 == 0 {
        // let avg_frame_time =
        // self.render_stats.total_render_time / self.render_stats.frame_count;
        // let fps = 1.0 / avg_frame_time.as_secs_f64();
        // trace(
        //     LOG_MODULE_NAME,
        //     &format!("Avg FPS: {:.1}, Frame time: {:?}", fps, avg_frame_time),
        // );
        // }
        // r.queue.submit(std::iter::once(fcx.encoder.finish()));
        // fcx.output.present();

        Ok(())
    }
}
