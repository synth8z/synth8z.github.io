use futures::channel::oneshot;
use gloo_timers::future::TimeoutFuture;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Document, Element, EventTarget};

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    spawn_local(async {
        if let Err(e) = run().await {
            web_sys::console::error_1(&e);
        }
    });
}

async fn run() -> Result<JsValue, JsValue> {
    let doc = document()?;

    // Grab DOM nodes
    let l1 = get(&doc, "l1")?;
    let l2 = get(&doc, "l2")?;
    let l3 = get(&doc, "l3")?;
    let t1 = get(&doc, "t1")?;
    let t2 = get(&doc, "t2")?;
    let t3 = get(&doc, "t3")?;
    let c1 = get(&doc, "c1")?;
    let c2 = get(&doc, "c2")?;
    let c3 = get(&doc, "c3")?;
    let block = get(&doc, "block")?;
    let finale = get(&doc, "finale")?;

    // Content
    let l1_text = " Amid a global arms race, AGI was achieved — quietly, then everywhere.";
    let l2_text = " Autonomous agents went mainstream. 2028 was the last year humans had a monopoly on labour.";
    let l3_text = " By 2030, synthetic corporations (\"synths\") — indistinguishable blends of humans and thinking machines — had become the norm.";

    // Timing
    let speed = 58u32;
    let gap_between_lines = 1500u32;
    let gap_before_fade = 1600u32;
    let gap_before_finale = 800u32;
    let fade_fallback_ms = 1600u32;

    add_class(&l1, "show")?;
    type_text(&t1, l1_text, speed, &c1).await?;
    TimeoutFuture::new(gap_between_lines).await;

    add_class(&l2, "show")?;
    type_text(&t2, l2_text, speed, &c2).await?;
    TimeoutFuture::new(gap_between_lines).await;

    add_class(&l3, "show")?;
    type_text(&t3, l3_text, speed, &c3).await?;
    TimeoutFuture::new(gap_before_fade).await;

    // Fade out the block and wait for transitionend
    add_class(&block, "fadeout")?;
    wait_for_transition_end(&block, fade_fallback_ms).await;

    TimeoutFuture::new(gap_before_finale).await;
    add_class(&finale, "show")?;

    Ok(JsValue::NULL)
}

async fn type_text(
    target: &Element,
    text: &str,
    ms_per_char: u32,
    caret: &Element,
) -> Result<(), JsValue> {
    caret.set_attribute("style", "visibility:visible")?;
    let chars: Vec<char> = text.chars().collect();
    for i in 0..=chars.len() {
        let s: String = chars.iter().take(i).collect();
        target.set_text_content(Some(&s));
        TimeoutFuture::new(ms_per_char).await;
    }
    caret.set_attribute("style", "visibility:hidden")?;
    Ok(())
}

fn add_class(el: &Element, class: &str) -> Result<(), JsValue> {
    el.class_list().add_1(class)
}

fn document() -> Result<Document, JsValue> {
    window()
        .ok_or_else(|| js_str("no window"))?
        .document()
        .ok_or_else(|| js_str("no document"))
}

fn get(doc: &Document, id: &str) -> Result<Element, JsValue> {
    doc.get_element_by_id(id)
        .ok_or_else(|| js_str(&format!("missing #{id}")))
}

fn js_str(s: &str) -> JsValue {
    JsValue::from_str(s)
}

/// Wait for a CSS `transitionend` on the given element; fallback to timeout.
async fn wait_for_transition_end(el: &Element, fallback_ms: u32) {
    // Create a oneshot channel that resolves when transitionend fires.
    let (tx, rx) = oneshot::channel::<()>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    let closure = Closure::<dyn FnMut(_)>::new({
        let tx = tx.clone();
        move |_event: web_sys::Event| {
            if let Some(sender) = tx.borrow_mut().take() {
                let _ = sender.send(());
            }
        }
    });

    // Attach listener
    let target: &EventTarget = el.unchecked_ref();
    let _ =
        target.add_event_listener_with_callback("transitionend", closure.as_ref().unchecked_ref());

    // Fallback timeout
    let fallback = TimeoutFuture::new(fallback_ms);

    // Wait for either transitionend or fallback timeout
    let rx_fut = rx;
    let done = futures::future::select(
        Box::pin(async move {
            let _ = rx_fut.await;
        }),
        Box::pin(async move {
            fallback.await;
        }),
    )
    .await;

    // Remove listener and leak closure only if necessary
    let _ = target
        .remove_event_listener_with_callback("transitionend", closure.as_ref().unchecked_ref());
    // Ensure closure is not dropped while event could still fire:
    closure.forget();

    match done {
        futures::future::Either::Left(_) => (), // transitionend fired
        futures::future::Either::Right(_) => (), // timeout
    }
}
