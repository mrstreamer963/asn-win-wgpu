use crate::WgpuGuiHandler;
use crate::data::LOG_MODULE_NAME;

use super::RenderManager;
use super::frame_context::WgpuFrameContext;
use super::wgpu_context::WgpuGraphContext;

use asn_gui_core::TAsnRenderManager;
use asn_logger::log::info;
use asn_logger::*;
use asn_winit::WinitWindow;

/// Ошибка инициализации менеджера рендеринга
#[derive(Debug)]
struct RenderManagerError(String);

impl std::fmt::Display for RenderManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderManager error: {}", self.0)
    }
}

impl std::error::Error for RenderManagerError {}

/// Создает ошибку RenderManagerError из строки
fn render_error(msg: &str) -> Box<dyn std::error::Error> {
    Box::new(RenderManagerError(msg.to_string()))
}

/// Проверяет, что менеджер рендеринга инициализирован, и возвращает мутабельную ссылку на контекст
fn ensure_initialized_mut<H>(
    manager: &mut RenderManager<H>,
) -> Result<&mut WgpuGraphContext, Box<dyn std::error::Error>>
where
    H: WgpuGuiHandler,
{
    manager
        .s
        .as_mut()
        .ok_or_else(|| render_error("manager not initialized"))
}

/// Блокирует обработчик GUI и возвращает мутабельную ссылку на него
fn lock_handler<H>(
    handler: &std::sync::Arc<std::sync::Mutex<H>>,
) -> Result<std::sync::MutexGuard<'_, H>, Box<dyn std::error::Error>>
where
    H: WgpuGuiHandler,
{
    handler
        .lock()
        .map_err(|e| render_error(&format!("handler cant unlock - {e}")))
}

impl<H> TAsnRenderManager for RenderManager<H>
where
    H: WgpuGuiHandler,
{
    type Window = WinitWindow;

    fn init(&mut self, w: std::sync::Arc<Self::Window>) -> Result<(), Box<dyn std::error::Error>> {
        info!("RenderManager init2...");

        let context = pollster::block_on(WgpuGraphContext::new(w)).map_err(|e| {
            m_error!("Failed to create GPU state: {e}");
            render_error(&format!("init error: {e}"))
        })?;

        {
            let mut h = lock_handler(&self.h)?;
            h.init(&context);
        }

        self.s = Some(context);
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        m_info!("resize {width}, {height}");
        let s = ensure_initialized_mut(self)?;
        s.resize(width, height)
            .map_err(|e| render_error(&format!("resize error - {e}")))?;
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Проверяем, что менеджер инициализирован
        let s = self
            .s
            .as_ref()
            .ok_or_else(|| render_error("manager not initialized"))?;

        // Создаем контекст кадра
        let mut fcx = WgpuFrameContext::new(&s.surface, &s.device)
            .map_err(|e| render_error(&format!("draw error - {e}")))?;

        // Обновляем и отрисовываем GUI
        {
            let mut h = lock_handler(&self.h)?;
            h.draw(&mut fcx);
        }

        // Отправляем команды рендеринга и выводим кадр на экран
        s.queue.submit(std::iter::once(fcx.encoder.finish()));
        s.queue.present(fcx.output);

        // Обновляем статистику рендеринга
        let frame_duration = fcx.frame_start.elapsed();
        self.render_stats.frame_count += 1;
        self.render_stats.total_render_time += frame_duration;

        // Логируем производительность каждые 60 кадров
        if self.render_stats.frame_count % 60 == 0 {
            let frame_count = self.render_stats.frame_count.min(u32::MAX as u64) as u32;
            let avg_frame_time = self
                .render_stats
                .total_render_time
                .checked_div(frame_count)
                .unwrap_or(web_time::Duration::new(0, 0));
            let fps = 1.0 / avg_frame_time.as_secs_f64();
            m_trace!("Avg FPS: {:.1}, Frame time: {:?}", fps, avg_frame_time);
        }

        Ok(())
    }

    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Получаем графический контекст
        let s = self
            .s
            .as_ref()
            .ok_or_else(|| render_error("manager not initialized"))?;

        {
            let mut h = lock_handler(&self.h)?;
            h.update(s);
        }

        // Запрашиваем перерисовку окна
        s.window.request_redraw();

        Ok(())
    }
}
