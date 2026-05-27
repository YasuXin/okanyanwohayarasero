use anyhow::anyhow;
use wasm_bindgen::{JsCast};
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, AudioDestinationNode, AudioNode};
use js_sys::ArrayBuffer;
use js_sys::futures::JsFuture;

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

pub enum LOOPING {
	NO,
	YES
}

pub fn play_sound(
	ctx: &AudioContext,
	buffer: &AudioBuffer,
	looping: LOOPING
) -> Result<(), anyhow::Error> {
	let track_source = create_track_source(ctx, buffer)?;

	if matches!(looping, LOOPING::YES) {
		track_source.set_loop(true);
	}

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
