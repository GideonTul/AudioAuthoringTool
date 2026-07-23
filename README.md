# AudioAuthoringTool
This project aims to provide effective audio middleware that allows for more immersive audio
and more complex audio design.

# Primary Feature Goals

1. __Game Engine Portability__
    * The ability to use this tool with any game engine is, perhaps, the highest priority.

2. __Adaptive Audio__
    * The goal is to allow developers to tie game variables (like player health, speed, or enemy count) to parameters within audio events. For instance, seamless transisions in boss music, e.g. phase 1 to phase 2 music transitions.

3. __3D Audio__
    * The goal here is immersion by providing positional audio capabilities.

4. __File Support__
    * Support for various audio file types, such as MP3, WAV, OGG, etc.

5. __Realtime Updates__
    * The ability to make changes in the tool and see them reflected in the game engine.

6. __GUI__
    * A simple graphical interface.

# Secondary Feature Goals

1. __Audio Effects__
    * Support for effects like reverb, delay, and compression.

2. __Geometry Obstruction__
    * Ducking and mixing audio to sound more realistic when the listener goes behind a wall.

3. __Snapshot System__
    * A system for predefining mixes and transitioning to each one smoothly.

# For Developers
* I recommend using [this website](https://rust-lang.org/learn/get-started/) for instructions
on getting Rust setup in your environment. Then just simply clone the repository.

* Once everything is setup, you can build code by the command: cargo build -p app-to-run

* You can run code by the command: cargo run -p app-to-run

* When adding a new folder to apps or crates, be sure to add it to the "members" list in AudioAuthoringTool/Cargo.toml

* When adding a dependency you can use: cargo add *dependency* --package app-to-add-dep

***Crates Planned for Use***
* *cpal*
* *symphonia*
