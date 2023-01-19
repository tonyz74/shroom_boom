#[allow(unused)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    PauseMenu,
    Gameplay,
    LevelTransition
}
