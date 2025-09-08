
В итоге пришли к такому варианту:
asn-winit -> asn-wgpu -> [wgpu-handler]draw(wgpu_context)

                          Arc<Mutex<[wgpu-handler]>>update(delta_time)

[asn-winit] -> [asn-wgpu]
            -> [wgpu-handler]

// как заполнять gui-компоненты до вызова init ?
// State -> Loaded/Unloaded
// Option -> Option<Element>
// FnOnce(GraphContext) -> new TAsnGuiHandler()
// Для примера сделаем решение с Option<Element>

Бесплатную музыку и звуки можно взять отсюда:
https://zvukogram.com/zvuk/88634/


Event handler -> (?)


asn-winit -> asn-wgpu -> [wgpu-handler]draw(wgpu_context)
    event...  -> [event-handler]


modules, некоторые из них - это то, что должно в будущем быть вынесено в отдельные репозитории


нужно ли тогда вообще переходить на workspace (?)


вынести работу с матрицами в отдельный модуль, добавить возможности смещения

не забыть про реализацию под web :)
в будущем подумать над hot-reload....

Для web нужно переходить на event_proxy для однократного асинхронного вызова инициализации окна

Перейдем на event_proxy (?) - будем передавать событие UserEvent::Exit (?)

