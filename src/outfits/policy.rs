use super::catalog::Outfit;

pub fn ordinary_request(mode: i32, engage_state: i32, conditions: &[String]) -> bool {
  mode == 2
    && engage_state == 0
    && !conditions.iter().any(|condition| {
      ["私服", "情報", "会話", "デモ", "エンゲージ", "変身"]
        .iter()
        .any(|blocked| condition.contains(blocked))
    })
}

pub fn compatible_dress(current_rig: &str, outfit: &Outfit) -> bool {
  current_rig == outfit.rig
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::outfits::catalog::{find, OUTFITS};

  #[test]
  fn panette_maps_to_her_original_berserker_dress_after_reclass() {
    let outfit = find("PID_パネトネ", 2).unwrap();
    assert_eq!(outfit.initial_class, "JID_ベルセルク");
    assert_eq!(outfit.dress, "uBody_Axe1AF_c453");
    assert!(compatible_dress("uRig_HumnF1", outfit));
    // No current job or Somniel selection is an input to this lookup.
    assert!(!compatible_dress("uRig_HumnM1", outfit));
    assert!(!compatible_dress("uRig_Tik1AT", outfit));
  }

  #[test]
  fn all_playable_identities_and_both_alear_genders_are_explicit() {
    assert_eq!(OUTFITS.len(), 42);
    let unique: std::collections::HashSet<_> = OUTFITS.iter().map(|x| (x.pid, x.gender)).collect();
    assert_eq!(unique.len(), 42);
    let identities: std::collections::HashSet<_> = OUTFITS.iter().map(|x| x.pid).collect();
    assert_eq!(identities.len(), 41);
    assert_eq!(find("PID_リュール", 1).unwrap().dress, "uBody_Drg0AM_c001");
    assert_eq!(find("PID_リュール", 2).unwrap().dress, "uBody_Drg0AF_c051");
    assert!(find("PID_パネトネ", 1).is_none());
    assert!(find("PID_不明", 2).is_none());
    assert!(find("PID_リュール", 0).is_none());
    assert!(find("PID_ロサード", 2).is_some());
    for outfit in OUTFITS {
      assert!(outfit.dress.starts_with("uBody_") && !outfit.dress.contains("Wear"));
    }
  }

  #[test]
  fn dlc_allies_do_not_use_enemy_dresses() {
    assert_eq!(find("PID_セレスティア", 2).unwrap().dress, "uBody_Msn0DF_c553b");
    assert_eq!(find("PID_グレゴリー", 1).unwrap().dress, "uBody_Mag1AM_c503b");
    assert_eq!(find("PID_マデリーン", 2).unwrap().dress, "uBody_Amr1AF_c554b");
    assert_eq!(find("PID_エル", 2).unwrap().dress, "uBody_Sds0AF_c099");
    assert_eq!(find("PID_ラファール", 1).unwrap().dress, "uBody_Sds0AM_c049");
  }

  #[test]
  fn incompatible_rigs_are_not_forced() {
    let ivy = find("PID_アイビー", 2).unwrap();
    assert!(compatible_dress("uRig_Lnd0DF", ivy));
    assert!(!compatible_dress("uRig_HumnF1", ivy));
  }

  #[test]
  fn battle_scope_excludes_other_scenes_and_engaged_forms() {
    assert!(ordinary_request(2, 0, &[]));
    for mode in [0, 1, 3, 4, -1] {
      assert!(!ordinary_request(mode, 0, &[]));
    }
    for state in 1..=4 {
      assert!(!ordinary_request(2, state, &[]));
    }
    for condition in ["私服", "情報", "会話", "デモ", "エンゲージ中", "超エンゲージ中", "変身"]
    {
      assert!(!ordinary_request(2, 0, &[condition.to_owned()]));
    }
  }
}
