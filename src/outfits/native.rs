use super::{policy, profile};
use core::{ffi::c_void, ptr};

pub type Object = *mut c_void;
pub type Method = *const c_void;

fn base() -> usize {
  unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize }
}

unsafe fn function<T: Copy>(offset: usize) -> T {
  // All call sites use function pointers pinned by offline NSO verification.
  unsafe { core::mem::transmute_copy(&(base() + offset)) }
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
  let Some(outfit) = crate::outfits::catalog::find(&pid, get_gender(unit, ptr::null())) else {
    return;
  };
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
