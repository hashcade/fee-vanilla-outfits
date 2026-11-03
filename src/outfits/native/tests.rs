use super::*;
use std::cell::RefCell;

#[derive(Default)]
struct Runtime {
  strings: Vec<Box<[usize]>>,
  allocations: usize,
  setters: usize,
}

std::thread_local! {
  static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default());
}

fn il_string(value: &str) -> Object {
  let units: Vec<u16> = value.encode_utf16().collect();
  let mut storage = vec![0usize; (0x14 + (units.len() + 1) * 2).div_ceil(8)].into_boxed_slice();
  let object = storage.as_mut_ptr().cast::<u8>();
  unsafe {
    object.add(0x10).cast::<i32>().write(units.len() as i32);
    ptr::copy_nonoverlapping(units.as_ptr(), object.add(0x14).cast::<u16>(), units.len());
  }
  RUNTIME.with(|runtime| runtime.borrow_mut().strings.push(storage));
  object.cast()
}

extern "C" fn allocate(length: i32, _: Method) -> Object {
  RUNTIME.with(|runtime| runtime.borrow_mut().allocations += 1);
  il_string(&"\0".repeat(length as usize))
}

extern "C" fn set_dress(result: Object, dress: Object, _: Method) {
  RUNTIME.with(|runtime| runtime.borrow_mut().setters += 1);
  unsafe { result.cast::<u8>().add(0x28).cast::<Object>().write(dress) };
}

extern "C" fn get_pid(unit: Object, _: Method) -> Object {
  unsafe { read(unit, 0) }
}

extern "C" fn get_gender(unit: Object, _: Method) -> i32 {
  unsafe { read(unit, 8) }
}

extern "C" fn get_state(unit: Object, _: Method) -> i32 {
  unsafe { read(unit, 16) }
}

pub(super) fn function_address(offset: usize) -> usize {
  match offset {
    profile::GET_PID => get_pid as *const () as usize,
    profile::DRESS_GENDER => get_gender as *const () as usize,
    profile::GET_STATE => get_state as *const () as usize,
    profile::STRING_NEW_SIZE => allocate as *const () as usize,
    profile::SET_DRESS => set_dress as *const () as usize,
    _ => panic!("unexpected native call: {offset:#x}"),
  }
}

fn result(pid: &str, rig: &str, dress: &str) -> [usize; 64] {
  let mut fields = std::array::from_fn(|index| 0x1000 + index);
  fields[0x10 / 8] = il_string(pid) as usize;
  fields[0x20 / 8] = il_string(rig) as usize;
  fields[0x28 / 8] = il_string(dress) as usize;
  fields
}

#[test]
fn both_dialogue_paths_change_only_the_dress_for_all_42_outfits() {
  for outfit in catalog::OUTFITS {
    for with_unit in [false, true] {
      RUNTIME.with(|runtime| *runtime.borrow_mut() = Runtime::default());
      let mut fields = result(outfit.pid, outfit.rig, "uBody_CurrentClass");
      let before = fields;
      let mut unit = [il_string(outfit.pid) as usize, outfit.gender as usize];
      let original_unit = unit;
      unsafe {
        if with_unit {
          apply_original_dress(unit.as_mut_ptr().cast(), fields.as_mut_ptr().cast());
        } else {
          apply_original_dress_from_result(fields.as_mut_ptr().cast());
        }
        assert_eq!(string(fields[0x28 / 8] as Object).as_deref(), Some(outfit.dress));
      }
      assert_eq!(unit, original_unit);
      for index in 0..fields.len() {
        if index != 0x28 / 8 {
          assert_eq!(fields[index], before[index], "{} field {:#x}", outfit.name, index * 8);
        }
      }
      RUNTIME.with(|runtime| {
        assert_eq!(runtime.borrow().allocations, 1);
        assert_eq!(runtime.borrow().setters, 1);
      });
    }
  }
}

#[test]
fn dialogue_leaves_unknown_incompatible_and_already_original_models_unchanged() {
  for (pid, rig, dress) in [
    ("PID_不明", "uRig_HumnF1", "uBody_Unknown"),
    ("GID_マルス", "uRig_HumnM1", "uBody_Emblem"),
    ("PID_パネトネ", "uRig_Tik1AT", "uBody_Transformation"),
    ("PID_パネトネ", "uRig_HumnM1", "uBody_Incompatible"),
    ("PID_パネトネ", "uRig_HumnF1", "uBody_Axe1AF_c453"),
  ] {
    RUNTIME.with(|runtime| *runtime.borrow_mut() = Runtime::default());
    let mut fields = result(pid, rig, dress);
    let before = fields;
    unsafe { apply_original_dress_from_result(fields.as_mut_ptr().cast()) };
    assert_eq!(fields, before);
    RUNTIME.with(|runtime| {
      assert_eq!(runtime.borrow().allocations, 0);
      assert_eq!(runtime.borrow().setters, 0);
    });
  }
}

#[test]
fn absent_dialogue_results_do_not_call_native_functions() {
  unsafe {
    apply_original_dress_from_result(ptr::null_mut());
    apply_original_dress(ptr::null_mut(), ptr::null_mut());
  }
}

#[test]
fn post_battle_kagetsu_keeps_swordmaster_clothes_after_reclassing_to_wyvern() {
  let mut fields = result("PID_カゲツ", "uRig_HumnM1", "current_wyvern_dress");
  fields[0x170 / 8] = il_string("AOC_Talk_c302") as usize;
  fields[0x180 / 8] = il_string("AOC_Hub_Hum2M") as usize;
  let before = fields;
  unsafe {
    apply_original_dress_from_result(fields.as_mut_ptr().cast());
    assert_eq!(string(fields[0x28 / 8] as Object).as_deref(), Some("uBody_Swd1AM_c302"));
  }
  for index in 0..fields.len() {
    if index != 0x28 / 8 {
      assert_eq!(fields[index], before[index]);
    }
  }
}

#[test]
fn native_scene_conditions_and_engage_state_are_preserved() {
  let mut array = [0usize; 6];
  array[0x18 / 8] = 2;
  array[0x20 / 8] = il_string("会話") as usize;
  array[0x28 / 8] = il_string("エンゲージ中") as usize;
  let before = array;
  let mut unit = [0usize, 0, 2];
  unsafe {
    assert_eq!(conditions(ptr::null_mut()), Some(Vec::new()));
    assert_eq!(
      conditions(array.as_mut_ptr().cast()),
      Some(vec!["会話".to_owned(), "エンゲージ中".to_owned()])
    );
    assert_eq!(engage_state(unit.as_mut_ptr().cast()), 2);
  }
  assert_eq!(array, before);
  assert_eq!(unit, [0, 0, 2]);
}

#[test]
fn opening_marker_uses_the_native_reference_not_its_spelling() {
  let marker = il_string("native opening condition");
  let same_text = il_string("native opening condition");
  let mut array = [0usize; 6];
  array[0x18 / 8] = 2;
  array[0x20 / 8] = il_string("デモ") as usize;
  array[0x28 / 8] = marker as usize;
  let before = array;
  unsafe {
    assert!(contains_condition(array.as_mut_ptr().cast(), marker));
    assert!(!contains_condition(array.as_mut_ptr().cast(), same_text));
    assert!(!contains_condition(array.as_mut_ptr().cast(), ptr::null_mut()));
    assert!(!contains_condition(ptr::null_mut(), marker));
    array[0x18 / 8] = 0;
    assert!(!contains_condition(array.as_mut_ptr().cast(), marker));
  }
  array[0x18 / 8] = 2;
  assert_eq!(array, before);
}

#[test]
fn opening_preserves_animation_weapons_and_other_fields_for_every_original_outfit() {
  for outfit in catalog::OUTFITS {
    RUNTIME.with(|runtime| *runtime.borrow_mut() = Runtime::default());
    let marker = il_string("native opening condition");
    let mut array = [0usize; 6];
    array[0x18 / 8] = 2;
    array[0x20 / 8] = il_string("デモ") as usize;
    array[0x28 / 8] = marker as usize;
    let original_array = array;
    let mut unit = [il_string(outfit.pid) as usize, outfit.gender as usize, 0];
    let original_unit = unit;
    let mut fields = result(outfit.pid, outfit.rig, "current_class_dress");
    let before = fields;
    unsafe {
      assert!(policy::battle_request(
        2,
        engage_state(unit.as_mut_ptr().cast()),
        contains_condition(array.as_mut_ptr().cast(), marker),
        &conditions(array.as_mut_ptr().cast()).unwrap(),
      ));
      apply_original_dress(unit.as_mut_ptr().cast(), fields.as_mut_ptr().cast());
      assert_eq!(string(fields[0x28 / 8] as Object).as_deref(), Some(outfit.dress));
    }
    for index in 0..fields.len() {
      if index != 0x28 / 8 {
        assert_eq!(fields[index], before[index], "{} field {:#x}", outfit.name, index * 8);
      }
    }
    assert_eq!(unit, original_unit);
    assert_eq!(array, original_array);
    RUNTIME.with(|runtime| assert_eq!(runtime.borrow().setters, 1));
  }
}

#[test]
fn opening_does_not_put_a_human_dress_on_a_transformed_rig() {
  RUNTIME.with(|runtime| *runtime.borrow_mut() = Runtime::default());
  let mut unit = [il_string("PID_パネトネ") as usize, 2, 0];
  let mut fields = result("PID_パネトネ", "uRig_Tik1AT", "native_dragon_dress");
  let before = fields;
  unsafe { apply_original_dress(unit.as_mut_ptr().cast(), fields.as_mut_ptr().cast()) };
  assert_eq!(fields, before);
  RUNTIME.with(|runtime| {
    assert_eq!(runtime.borrow().allocations, 0);
    assert_eq!(runtime.borrow().setters, 0);
  });
}
