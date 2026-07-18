*  Не хранить массив разных компонентов как Vec<Box<dyn WgpuComponent>> из-за потерь в dyn,
а вместо этого добавить handler в State, у которого вызывать функции init(WgpuContext), start_draw(), end_draw()

* Исправить asn_log вызов функции инициализаци логов один раз через get_or_init

* Заменить tokio на use futures::channel::mpsc ? - mpmc 

Возможно реализовать mpsc реализовать для actor с почтовыми ящиками
