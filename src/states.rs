#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    LoadAssets,
    BoardSetup,
    TurnPlanning,
    TurnRun
}