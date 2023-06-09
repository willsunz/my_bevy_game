enum GameState {
    #[default]
    Splash,
    Menu,
    NewGame,
    Game,
}

enum MenuState {
    Main,
    NewGame,
    PlayMenu,
    FeedMenu,
    Settings,
    Update,
    Transcation,
    #[default]
    Disabled,
}

enum MenuButtonAction {
    NewGame, //Create a new Game
    ContinueGame, //Continue the Game
    Feed, //Feed pet
    Food, //Feed food 
    Water,//Feed water
    Play, //Play with pet
    Wake, //Wake up pet
    Sleep,//Make pet sleep
    Idle, //Make pet into idle situation
    Settings,//Game settings
    Update, //Pet state update
    Transation,//Buy or Sell pet
    BackToMainMenu,    
    Quit,
}



    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;
    struct OnPlayMenuScreen;
    struct OnFeedMenuScreen;
    struct OnSettingsMenuScreen;
    struct OnUpdateScreen;
    struct OnTransactionScreen;