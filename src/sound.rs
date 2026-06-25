use anyhow::anyhow;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, AudioDestinationNode, AudioNode};
use js_sys::ArrayBuffer;
use js_sys::futures::JsFuture;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::browser;

pub fn create_audio_context() -> Result<AudioContext, anyhow::Error> {
	AudioContext::new().map_err(|err| {
		anyhow!("sound : create_audio_context() : Could not create audio context: {:?}", err)
	})
}

fn create_buffer_source(ctx: &AudioContext) -> Result<AudioBufferSourceNode, anyhow::Error> {
	ctx.create_buffer_source().map_err(|err|
		anyhow!("sound : create_buffer_source() : Error creating buffer source: {:?}", err)
	)
}

fn connect_with_audio_node(
	buffer_source: &AudioBufferSourceNode,
	destination: &AudioDestinationNode
) -> Result<AudioNode, anyhow::Error> {
	buffer_source.connect_with_audio_node(&destination).map_err(|err|
		anyhow!("sound : connect_with_audio_node() : Error connecting audio source to destination: {:?}", err)
	)
}

fn create_track_source(
	ctx: &AudioContext,
	buffer: &AudioBuffer,
) -> Result<AudioBufferSourceNode, anyhow::Error> {
	let track_source = create_buffer_source(ctx)?;
	track_source.set_buffer(Some(&buffer));
	connect_with_audio_node(&track_source, &ctx.destination())?;

	Ok(track_source)
}
/*
pub enum LOOPING {
	NO,
	YES
}*/

pub fn play_sound(
	ctx: &AudioContext,
	buffer: &AudioBuffer,
) -> Result<(), anyhow::Error> {
	let track_source = create_track_source(ctx, buffer)?;

	/*if matches!(looping, LOOPING::YES) {
		track_source.set_loop(true);
	}*/

	track_source
		.start_with_when(0.0)
		.map_err(|err| anyhow!("sound : play_sound : Could not starting sound: {:?}", err))

}

pub async fn decode_audio_data(
	ctx: &AudioContext,
	array_buffer: &ArrayBuffer,
) -> Result<AudioBuffer, anyhow::Error> {
	JsFuture::from(
		ctx.decode_audio_data(array_buffer)
			.map_err(|err| anyhow!("sound : decode_audio_data() : Could not decode audio from ArrayBuffer: {:?}", err))?
	)
		.await
		.map_err(|err| anyhow!("sound : decode_audio_data() : Error convert promise to future {:#?}", err))?
		.dyn_into::<AudioBuffer>()
		.map_err(|err| anyhow!("sound : decode_audio_data() : Could not cast into AudioBuffer: {:?}", err))
}

#[wasm_bindgen]
pub fn play_audio_js(audio_class_name: &str) -> Result<(), JsValue> {
	let document = browser::document().expect("failed to get browser document");

	let element = document.get_elements_by_class_name(audio_class_name).get_with_index(0).expect("no node has class_name");
	let cloned_element = element.clone_node()?;

	let audios_element = document.get_element_by_id("audios").unwrap();

	let _ = audios_element.append_child(cloned_element.as_ref())?;

	let audio_element = cloned_element.dyn_into::<web_sys::HtmlAudioElement>()?;

	let _ = audio_element.play()?;

	Ok(())
}

#[wasm_bindgen]
pub fn delete_audio_js(audio_class_name: &str) -> Result<(), JsValue> {
	let document = browser::document().expect("failed to get browser document");

	let audios_element = document.get_element_by_id("audios").unwrap();
	let element = audios_element.get_elements_by_class_name(audio_class_name);

	for i in 0..element.length() {

		if let Some(item) = element.item(i) {
			let _ = audios_element.remove_child(&item);
		}
	}

	Ok(())
}