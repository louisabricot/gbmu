struct GameBoy {
    model: Model,
    speed: SpeedMode,
}

enum SpeedMode {
    DOUBLE,
    NORMAL,
}

enum Model {
    DMG,
    CGB,
}
