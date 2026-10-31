#![deny(unsafe_op_in_unsafe_fn)]

pub mod outfits;

#[cfg(target_os = "switch")]
#[skyline::main(name = "feeoutfits")]
pub fn skyline_main() {
  if supported_game() {
    outfits::plugin::install();
  }
}

#[cfg(target_os = "switch")]
fn supported_game() -> bool {
  use skyline::hooks::{getRegionAddress, Region};
  let mut version = skyline::nn::oe::DisplayVersion { name: [0; 16] };
  unsafe {
    skyline::nn::oe::GetDisplayVersion(&mut version);
  }
  if version.name[..6] != [b'2', b'.', b'0', b'.', b'0', 0] {
    return false;
  }
  let base = unsafe { getRegionAddress(Region::Text) as usize };
  let rodata = unsafe { getRegionAddress(Region::Rodata) as usize };
  if base == 0 || rodata.checked_sub(base) != Some(RODATA_OFFSET) {
    return false;
  }
  unsafe { core::slice::from_raw_parts((base + BUILD_ID_OFFSET) as *const u8, 16) == BUILD_ID }
}

pub const BUILD_ID_OFFSET: usize = 0x05D7B20C;
pub const RODATA_OFFSET: usize = 0x03F48000;
pub const BUILD_ID: [u8; 16] =
  [0x8C, 0x08, 0xB9, 0x71, 0x9E, 0x08, 0x5F, 0x91, 0x84, 0x7B, 0x5E, 0x0F, 0x93, 0x5D, 0x94, 0x88];
