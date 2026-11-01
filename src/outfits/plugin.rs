use super::{
  native::{self, Method, Object},
  policy, profile,
  scope::{self, CombatScope},
};

pub fn install() {
  skyline::install_hooks!(
    create_appearance,
    get_from_unit,
    get_unit_info,
    get_for_talk_pid,
    get_for_talk_unit
  );
}

#[skyline::hook(offset = profile::APPEARANCE)]
fn create_appearance(status: Object, distance: i32, conditions: Object, method: Method) -> Object {
  let unit = unsafe { native::read::<Object>(status, 0x28) };
  let _scope = CombatScope::enter(unit as usize);
  call_original!(status, distance, conditions, method)
}

#[skyline::hook(offset = profile::GET_FROM_UNIT)]
fn get_from_unit(
  mode: i32,
  unit: Object,
  equipped: Object,
  conditions: Object,
  method: Method,
) -> Object {
  let result = call_original!(mode, unit, equipped, conditions, method);
  if result.is_null() || !scope::contains(unit as usize) {
    return result;
  }
  let Some(conditions) = (unsafe { native::conditions(conditions) }) else {
    return result;
  };
  let state = unsafe { native::engage_state(unit) };
  if policy::ordinary_request(mode, state, &conditions) {
    unsafe {
      native::apply_original_dress(unit, result);
    }
  }
  result
}

#[skyline::hook(offset = profile::UNIT_INFO)]
fn get_unit_info(unit: Object, equipped: Object, method: Method) -> Object {
  let result = call_original!(unit, equipped, method);
  if !unit.is_null() && !result.is_null() && unsafe { native::engage_state(unit) } == 0 {
    unsafe {
      native::apply_original_dress(unit, result);
    }
  }
  result
}

#[skyline::hook(offset = profile::TALK_PID)]
fn get_for_talk_pid(pid: Object, method: Method) -> Object {
  let result = call_original!(pid, method);
  unsafe {
    native::apply_original_dress_from_result(result);
  }
  result
}

#[skyline::hook(offset = profile::TALK_UNIT)]
fn get_for_talk_unit(unit: Object, method: Method) -> Object {
  let result = call_original!(unit, method);
  if !unit.is_null() && !result.is_null() && unsafe { native::engage_state(unit) } == 0 {
    unsafe {
      native::apply_original_dress(unit, result);
    }
  }
  result
}
