//! Original battle clothes, not Somniel accessories. Exact native PID/dress pairs.

#[derive(Debug)]
pub struct Outfit {
  pub name: &'static str,
  pub pid: &'static str,
  pub gender: i32,
  pub initial_class: &'static str,
  pub dress: &'static str,
  pub rig: &'static str,
}

macro_rules! outfit {
  ($name:literal, $pid:literal, $gender:literal, $class:literal, $dress:literal, $rig:literal) => {
    Outfit {
      name: $name,
      pid: $pid,
      gender: $gender,
      initial_class: $class,
      dress: $dress,
      rig: $rig,
    }
  };
}

pub const OUTFITS: &[Outfit] = &[
  outfit!("Alear", "PID_リュール", 1, "JID_神竜ノ子", "uBody_Drg0AM_c001", "uRig_HumnM1"),
  outfit!("Alear", "PID_リュール", 2, "JID_神竜ノ子", "uBody_Drg0AF_c051", "uRig_HumnF1"),
  outfit!("Vander", "PID_ヴァンドレ", 1, "JID_パラディン", "uBody_Cav1BM_c500", "uRig_HumnM1"),
  outfit!("Framme", "PID_フラン", 2, "JID_モンク", "uBody_Rod0AF_c550", "uRig_HumnF1"),
  outfit!("Clanne", "PID_クラン", 1, "JID_マージ", "uBody_Mag0AM_c501", "uRig_HumnM1"),
  outfit!(
    "Alfred",
    "PID_アルフレッド",
    1,
    "JID_アヴニール下級",
    "uBody_Avn0BM_c100",
    "uRig_HumnM1"
  ),
  outfit!(
    "Boucheron",
    "PID_ブシュロン",
    1,
    "JID_アクスファイター",
    "uBody_Axe0AM_c101",
    "uRig_HumnM1"
  ),
  outfit!("Etie", "PID_エーティエ", 2, "JID_アーチャー", "uBody_Bow0AF_c152", "uRig_HumnF1"),
  outfit!(
    "Céline",
    "PID_セリーヌ",
    2,
    "JID_フロラージュ下級",
    "uBody_Flr0AF_c150",
    "uRig_HumnF1High"
  ),
  outfit!("Louis", "PID_ルイ", 1, "JID_ランスアーマー", "uBody_Amr0AM_c102", "uRig_HumnM1"),
  outfit!("Chloé", "PID_クロエ", 2, "JID_ランスペガサス", "uBody_Wng0EF_c153", "uRig_HumnF1"),
  outfit!(
    "Diamant",
    "PID_ディアマンド",
    1,
    "JID_スュクセサール下級",
    "uBody_Scs0AM_c200",
    "uRig_HumnM1"
  ),
  outfit!("Amber", "PID_アンバー", 1, "JID_ランスナイト", "uBody_Cav0BM_c203", "uRig_HumnM1"),
  outfit!("Jade", "PID_ジェーデ", 2, "JID_アクスアーマー", "uBody_Amr0AF_c250", "uRig_HumnF1"),
  outfit!(
    "Alcryst",
    "PID_スタルーク",
    1,
    "JID_ティラユール下級",
    "uBody_Trl0AM_c201",
    "uRig_HumnM1"
  ),
  outfit!("Lapis", "PID_ラピス", 2, "JID_ソードファイター", "uBody_Swd0AF_c251", "uRig_HumnF1"),
  outfit!("Citrinne", "PID_シトリニカ", 2, "JID_マージ", "uBody_Mag0AF_c252", "uRig_HumnF1"),
  outfit!("Ivy", "PID_アイビー", 2, "JID_リンドブルム下級", "uBody_Lnd0DF_c350", "uRig_Lnd0DF"),
  outfit!("Zelkov", "PID_ゼルコバ", 1, "JID_シーフ", "uBody_Dge0AM_c301", "uRig_HumnM1"),
  outfit!("Kagetsu", "PID_カゲツ", 1, "JID_ソードマスター", "uBody_Swd1AM_c302", "uRig_HumnM1"),
  outfit!(
    "Hortensia",
    "PID_オルテンシア",
    2,
    "JID_スレイプニル下級",
    "uBody_Slp0EF_c351",
    "uRig_HumnF1"
  ),
  outfit!("Rosado", "PID_ロサード", 2, "JID_ドラゴンナイト", "uBody_Wng2DF_c303", "uRig_HumnF1"),
  outfit!(
    "Goldmary",
    "PID_ゴルドマリー",
    2,
    "JID_ブレイブヒーロー",
    "uBody_Swd2AF_c352",
    "uRig_HumnF1"
  ),
  outfit!(
    "Timerra",
    "PID_ミスティラ",
    2,
    "JID_ピッチフォーク下級",
    "uBody_Pcf0AF_c450",
    "uRig_HumnF1"
  ),
  outfit!("Merrin", "PID_メリン", 2, "JID_ウルフナイト", "uBody_Cav2CF_c452", "uRig_HumnF1"),
  outfit!("Panette", "PID_パネトネ", 2, "JID_ベルセルク", "uBody_Axe1AF_c453", "uRig_HumnF1"),
  outfit!("Fogado", "PID_フォガート", 1, "JID_クピードー下級", "uBody_Cpd0BM_c400", "uRig_HumnM1"),
  outfit!("Pandreo", "PID_パンドロ", 1, "JID_ハイプリースト", "uBody_Rod2AM_c401", "uRig_HumnM1"),
  outfit!("Bunet", "PID_ボネ", 1, "JID_グレートナイト", "uBody_Amr2BM_c402", "uRig_HumnM1"),
  outfit!("Anna", "PID_アンナ", 2, "JID_アクスファイター", "uBody_Axe0AF_c552", "uRig_HumnF1"),
  outfit!("Jean", "PID_ジャン", 1, "JID_モンク", "uBody_Rod0AM_c103", "uRig_HumnM1"),
  outfit!("Yunaka", "PID_ユナカ", 2, "JID_シーフ", "uBody_Dge0AF_c253", "uRig_HumnF1"),
  outfit!("Seadall", "PID_セアダス", 1, "JID_ダンサー", "uBody_Dnc0AM_c403", "uRig_HumnM1"),
  outfit!("Lindon", "PID_リンデン", 1, "JID_セイジ", "uBody_Mag1AM_c304", "uRig_HumnM1"),
  outfit!("Saphir", "PID_ザフィーア", 2, "JID_ウォーリアー", "uBody_Axe2AF_c254", "uRig_HumnF1"),
  outfit!("Veyle", "PID_ヴェイル", 2, "JID_邪竜ノ娘", "uBody_Sdp0AF_c551", "uRig_HumnF1Low"),
  outfit!("Mauvier", "PID_モーヴ", 1, "JID_ロイヤルナイト", "uBody_Lnc2BM_c502", "uRig_HumnM1"),
  outfit!("Nel", "PID_エル", 2, "JID_裏邪竜ノ娘", "uBody_Sds0AF_c099", "uRig_HumnF1"),
  outfit!("Rafal", "PID_ラファール", 1, "JID_裏邪竜ノ子", "uBody_Sds0AM_c049", "uRig_HumnM1"),
  outfit!(
    "Zelestia",
    "PID_セレスティア",
    2,
    "JID_メリュジーヌ_味方",
    "uBody_Msn0DF_c553b",
    "uRig_HumnF1"
  ),
  outfit!("Gregory", "PID_グレゴリー", 1, "JID_セイジ", "uBody_Mag1AM_c503b", "uRig_HumnM1"),
  outfit!("Madeline", "PID_マデリーン", 2, "JID_ジェネラル", "uBody_Amr1AF_c554b", "uRig_HumnF1"),
];

pub fn find(pid: &str, dress_gender: i32) -> Option<&'static Outfit> {
  OUTFITS.iter().find(|outfit| outfit.pid == pid && outfit.gender == dress_gender)
}
