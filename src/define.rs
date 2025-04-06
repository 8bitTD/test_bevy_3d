pub mod common{
    pub const TOOLNAME: &str = "test_bevy_3d";

}

pub mod system{
    pub const FPS: f32 = 60.0;
}

pub mod assets{
    pub const DEFAULTFONT: &str = "fonts/NotoSansJP-Bold.ttf";

    pub const BGM: &str = "bgm/maou_bgm_8bit29.mp3";
    //pub const BGM: &str = "bgm/maou_bgm_8bit25.mp3";
    pub const BGMENDING: &str = "bgm/ending.mp3";

    pub const SOUNDJUMP: &str = "sound/jump.mp3";
    pub const SOUNDGRAB: &str = "sound/grab.wav";
    pub const SOUNDDEATH: &str = "sound/se_hit_002.wav";
    pub const SOUNDENTER: &str = "sound/se_powerup_005.wav";
}

pub mod value{
    pub const VOLUME: f32 = 0.05; //0.05
    pub const FADETIME: f32 = 1.0;
    pub const DEFAULTTEXTSTAGEALPHA: f32 = 3.0;
    pub const ENDINGTEXTMOVE: f32 = 130.0;
    pub const TUTORIALBLINKTIMER: f32 = 0.1;
    pub const TUTORIALMOUSEMOVETIMER: f32 = 0.2;
    pub const DEFAULTROPEDISTANCE: f32 = 250.0;
    pub const FACIALBLINK: f32 = 0.1;
    pub const STAGEPATH: &str = "./assets/stage/stage_";
    pub const RESETRANGE: f32 = 520.0;
}

pub mod debug{
    pub const STARTSTAGE: usize = 1;//スタートステージ
    pub const MAXSTAGE: usize = 7;//最大ステージ数
    pub const ISCLEAR: bool = false;//初期化クリアフラグ
    pub const RAPIERDEBUGRENDERPLUGINENABLED: bool = true;//物理のガイド表示
    pub const ISSKIPTUTORIAL: bool = false;//チュートリアルスキップ
}