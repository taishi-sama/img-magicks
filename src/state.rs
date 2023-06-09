use crate::img2br::ImgToBraille;

#[derive(Clone)]
pub enum AppState {
    None,
    ImgToBraille(ImgToBraille),
}
