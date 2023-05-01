
use image::{
  codecs::gif::GifDecoder,
  Frame, 
  AnimationDecoder
};

use bevy::{
  prelude::*,
  render::render_resource::*
};

#[derive(Component)]
pub struct JeffComponent {
  frames: Vec<Frame>,
  frame_index: usize,
  images: Vec<Handle<Image>>,
  width: u32,
  height: u32
}

impl JeffComponent {

  pub fn new(
    url: String,
    asset_server: Res<AssetServer>
  ) -> Self {

    let mut downloaded: Vec<u8> = vec![];
    for byte in reqwest::blocking::get(url).unwrap().bytes().unwrap() {
      downloaded.push(byte);
    }

    let decoder = GifDecoder::new(&downloaded as &[u8]).unwrap();
    let frames = decoder.into_frames().collect_frames().expect("Can't decode frames.");

    let images: Vec<Handle<Image>> = vec![];
    for frame in frames {

      let image: Image = Image::new(
        Extent3d { 
          width: frame.buffer().width(),
          height: frame.buffer().height(),
          depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        frame.buffer().into_raw(),
        TextureFormat::Rgba32Float
      );

      //images.push(Handle::)
    
    }

    Self { 
      width: frames[0].buffer().width(),
      height: frames[0].buffer().height(),
      frames,
      images,
      frame_index: 0,
    }

  }

}

pub fn handle_jeff_components(
  mut jeff_query: Query<(&mut UiImage, &mut JeffComponent)>,
  asset_server: Res<AssetServer>
) {

  for (mut jeff_image, mut jeff) in jeff_query.iter_mut() {
    
    jeff.frame_index += 1;
  }
}