use super::{catalog, policy, profile};
use core::{ffi::c_void, ptr};

pub type Object = *mut c_void;
pub type Method = *const c_void;

#[cfg(not(test))]
fn base() -> usize {
  unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize }
}

unsafe fn function<T: Copy>(offset: usize) -> T {
  // All call sites use function pointers pinned by offline NSO verification.
  #[cfg(not(test))]
  let address = base() + offset;
  #[cfg(test)]
  let address = tests::function_address(offset);
  unsafe { core::mem::transmute_copy(&address) }
}

pub unsafe fn read<T: Copy + Default>(object: Object, offset: usize) -> T {
  if object.is_null() {
    return T::default();
  }
  unsafe { object.cast::<u8>().add(offset).cast::<T>().read() }
}

unsafe fn string(object: Object) -> Option<String> {
  if object.is_null() {
    return None;
  }
  let length: i32 = unsafe { read(object, 0x10) };
  if length < 0 {
    return None;
  }
  let chars = unsafe {
    core::slice::from_raw_parts(object.cast::<u8>().add(0x14).cast::<u16>(), length as usize)
  };
  String::from_utf16(chars).ok()
}

pub unsafe fn conditions(array: Object) -> Option<Vec<String>> {
  if array.is_null() {
    return Some(Vec::new());
  }
  let length: usize = unsafe { read(array, 0x18) };
  (0..length).map(|index| unsafe { string(read(array, 0x20 + index * 8)) }).collect()
}

unsafe fn contains_condition(array: Object, condition: Object) -> bool {
  if array.is_null() || condition.is_null() {
    return false;
  }
  let length: usize = unsafe { read(array, 0x18) };
  (0..length).any(|index| unsafe { read::<Object>(array, 0x20 + index * 8) == condition })
}

#[cfg(target_os = "switch")]
pub unsafe fn is_engage_start(array: Object) -> bool {
  // The native sequence resolves this literal before building its condition
  // array. CreateFromGameStatus copies the same references when merging it.
  let cell = unsafe { ((base() + profile::ENGAGE_START_LITERAL) as *const Object).read() };
  let condition: Object = unsafe { read(cell, 0) };
  unsafe { contains_condition(array, condition) }
}

pub unsafe fn engage_state(unit: Object) -> i32 {
  let get: extern "C" fn(Object, Method) -> i32 = unsafe { function(profile::GET_STATE) };
  get(unit, ptr::null())
}

/// Replace only the transient dress reference. Never mutate a unit or job.
pub unsafe fn apply_original_dress(unit: Object, result: Object) {
  if unit.is_null() || result.is_null() {
    return;
  }
  let get_pid: extern "C" fn(Object, Method) -> Object = unsafe { function(profile::GET_PID) };
  let get_gender: extern "C" fn(Object, Method) -> i32 = unsafe { function(profile::DRESS_GENDER) };
  let Some(pid) = (unsafe { string(get_pid(unit, ptr::null())) }) else { return };
  let Some(outfit) = catalog::find(&pid, get_gender(unit, ptr::null())) else {
    return;
  };
  unsafe { apply_dress(outfit, result) };
}

/// PID-based dialogue/exploration requests use the resolved rig to select Alear's gender.
pub unsafe fn apply_original_dress_from_result(result: Object) {
  let Some(pid) = (unsafe { string(read(result, 0x10)) }) else { return };
  let Some(rig) = (unsafe { string(read(result, 0x20)) }) else { return };
  let Some(outfit) = catalog::find_by_rig(&pid, &rig) else { return };
  unsafe { apply_dress(outfit, result) };
}

unsafe fn apply_dress(outfit: &catalog::Outfit, result: Object) {
  let Some(rig) = (unsafe { string(read(result, 0x20)) }) else { return };
  if !policy::compatible_dress(&rig, outfit) {
    return;
  }
  if unsafe { string(read(result, 0x28)) }.as_deref() == Some(outfit.dress) {
    return;
  }
  let units: Vec<u16> = outfit.dress.encode_utf16().collect();
  let allocate: extern "C" fn(i32, Method) -> Object =
    unsafe { function(profile::STRING_NEW_SIZE) };
  let dress = allocate(units.len() as i32, ptr::null());
  if dress.is_null() {
    return;
  }
  unsafe {
    ptr::copy_nonoverlapping(
      units.as_ptr(),
      dress.cast::<u8>().add(0x14).cast::<u16>(),
      units.len(),
    );
  }
  // The native setter includes the IL2CPP write barrier.
  let set: extern "C" fn(Object, Object, Method) = unsafe { function(profile::SET_DRESS) };
  set(result, dress, ptr::null());
}

#[cfg(test)]
mod tests;
