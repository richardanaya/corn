use crate::browser::*;
use ref_thread_local::RefThreadLocal;

extern "C" {
    fn CanvasRenderingContext2D_clearRect(ctx: i32, x: i32, y: i32, w: i32, h: i32);
    fn CanvasRenderingContext2D_drawImage(ctx: i32, i: i32, x: i32, y: i32);

}

ref_thread_local! {
    static managed WINDOW: i32 = get_window();
    static managed DOCUMENT: i32 = {
        let win = get_window();
        get_document(win)
    };
    static managed CTX: i32 = {
        let win = get_window();
        let doc = get_document(win);
        let screen = query_selector(doc,"#screen");
        get_context(screen,"2d")
    };
}

pub fn load_image(src: &str) -> i32 {
    let doc = *DOCUMENT.borrow();
    query_selector(doc, &format!("#{}", src))
}

pub fn draw_image(i: i32, x: i32, y: i32) {
    let ctx = *CTX.borrow();
    unsafe {
        CanvasRenderingContext2D_drawImage(ctx, i, x, y);
    }
}

pub fn clear(x: i32, y: i32, w: i32, h: i32) {
    let ctx = *CTX.borrow();
    unsafe {
        CanvasRenderingContext2D_clearRect(ctx, x, y, w, h);
    }
}
