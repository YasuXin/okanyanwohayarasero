use std::future::Future;
use anyhow::anyhow;
use js_sys::ArrayBuffer;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::{Closure, WasmClosure, WasmClosureFnOnce};

use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, HtmlImageElement, Response, Window};


//--------------------------------------------------------------
// ブラウザ関係の処理を定義する
//--------------------------------------------------------------


macro_rules! log {
	( $( $t:tt )* ) => {
		web_sys::console::log_1(&format!($( $t ) *).into());
	}
}

// 現在の時刻を取得
pub fn now() -> Result<f64, anyhow::Error> {
	Ok(
		window()?
			.performance()
			.ok_or_else(|| anyhow!("browser : now() : performance object not found"))?
			.now()
	)
}

pub fn window() -> Result<Window, anyhow::Error> {
	web_sys::window()
		.ok_or_else(|| anyhow!("browser : window() : No Window Found"))
}

pub fn document() -> Result<Document, anyhow::Error> {
	window()?
		.document()
		.ok_or_else(|| anyhow!("browser : document() : No Document Found"))
}

pub fn canvas() -> Result<HtmlCanvasElement, anyhow::Error> {
	document()?
		.get_element_by_id("canvas")
		.ok_or_else(|| anyhow!("browser : canvas() : No Canvas Element found with ID 'canvas"))?
		.dyn_into::<HtmlCanvasElement>() // JavaScriptから渡されたElementを、HtmlCanvasElementにキャストする
		.map_err(|element| anyhow!("browser : canvas() : Error converting {:?} to HtmlCanvasElement", element))
}

pub fn context() -> Result<CanvasRenderingContext2d, anyhow::Error> {
	canvas()?
		.get_context("2d")
		.map_err(|js_value| anyhow!("browser : context() : Error getting 2d context {:?}", js_value))?
		.ok_or_else(|| anyhow!("No 2d context found"))?
		.dyn_into::<CanvasRenderingContext2d>()
		.map_err(|element| anyhow!("browser : context() : Error converting {:?} to CanvasRenderingContext2d", element))
}

pub fn spawn_local<F> (future: F)
where
	F: Future<Output = ()> + 'static,
{
	wasm_bindgen_futures::spawn_local(future);
}

pub async fn fetch_with_str(resource: &str) -> Result<JsValue, anyhow::Error> {
	JsFuture::from(
		window()?
			.fetch_with_str(resource))
		.await
		.map_err(|err| anyhow!("browser : fetch_with_str() : Error fetching {:?}", err))
}


pub async fn fetch_response(resource: &str) -> Result<Response, anyhow::Error> {
	fetch_with_str(resource)
		.await?
		.dyn_into()
		.map_err(|err| anyhow!("browser : fetch_response() : Error converting fetch to Response {:?}", err))
}

pub async fn fetch_json(json_fetch: &str) -> Result<JsValue, anyhow::Error> {

	let resp = fetch_response(json_fetch).await?;

	JsFuture::from(
		resp.json()
			.map_err(| err | anyhow!("browser : fetch_json() : Could not get JSON from response {:#?}", err))?
	)
		.await
		.map_err(|err| anyhow!("browser : fetch_json() : Error fetching Json {:#?} to Response", err))
}

pub async fn fetch_array_buffer(resource: &str) -> Result<ArrayBuffer, anyhow::Error> {

	let array_buffer = fetch_response(resource)
		.await?
		.array_buffer()
		.map_err(| err | anyhow!("browser: fetch_array_buffer() : Error loading array buffer {:#?}", err))?;

	JsFuture::from(array_buffer)
		.await
		.map_err(|err| anyhow!("browser: fetch_array_buffer() : Error converting array buffer into future {:#?}", err))?
		.dyn_into()
		.map_err(|err| anyhow!("browser: fetch_array_buffer() : Error converting raw JsValue to ArrayBuffer {:#?}", err))
}

pub fn new_image() -> Result<HtmlImageElement, anyhow::Error> {
	HtmlImageElement::new()
		.map_err(|err| anyhow!("browser : new_image() : Could not create HtmlImageElement {:#?}", err))
}

pub fn closure_once<F, A, R, T> (fn_once: F) -> Closure<T, >
where
	F: 'static + WasmClosureFnOnce<T, A, R>,
	T: WasmClosure + ?Sized,
{
	Closure::once(fn_once)
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32, anyhow::Error> {
	window()?
		.request_animation_frame(callback.as_ref().unchecked_ref())
		.map_err(|err| anyhow!("browser : request_animation_frame() : Could not request animation frame {:#?}", err))
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
	Closure::wrap(data)
}

pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
	closure_wrap(Box::new(f))
}

pub fn find_element_by_id(id: &str) -> Result<Element, anyhow::Error> {
	document()
		.and_then(|doc| {
			doc.get_element_by_id(id)
				.ok_or_else(|| anyhow!("browser : draw_ui() : UI element not found"))
		})
}

pub fn insert_adjacent_html(id: &str, html: String) -> Result<(), anyhow::Error> {
	find_element_by_id(id)?
		.insert_adjacent_html("beforeend", &html)
		.map_err(|err| anyhow!("browser : insert_adjacent_html() : Could not insert html {:?}", err))

}

pub fn remove_first_child(id: &str) -> Result<(), anyhow::Error> {
	let ui = find_element_by_id(id)?;
	
	if let Some(child) = ui.first_child() {
		ui.remove_child(&child)
			.map(|_removed_child| ()) // remove_child() の戻り値が 削除された Node なので、Node を () に置き換える
			.map_err(|err| anyhow!("browser : remove_first_child() : Failed to remove child {:#?}", err))
			.and_then(|_unit| {
				canvas()?
					.focus()
					.map_err(|err| anyhow!("browser :remove_first_child() : Failed to focus {:#?}", err))
			})
	} else {
		Ok(())
	}
}

pub fn set_class_name_by_id(id: &str, class_name: &str) -> Result<(), anyhow::Error> {
	Ok(find_element_by_id(id).and_then(|element| {
		element.dyn_into::<HtmlElement>()
			.map_err(|err| anyhow!("browser : set_class_name_by_id() : Failed to remove child {:#?}", err))
	})?
		.set_class_name(class_name))
}

pub fn get_canvas_width() -> Result<u32, anyhow::Error> {
	Ok(find_element_by_id("canvas-area").and_then(|element| {
		element.dyn_into::<HtmlElement>()
		.map_err(|err| anyhow!("browser : get_canvas_width() : Failed to remove child {:#?}", err))
	})?
		.client_width() as u32
	)
}