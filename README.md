# Author's note
Dodge the Creeps, modified to use `bevy_ecs` and no collision.

This is taken from the April 20, 2021 master branch of godot-demo-projects Dodge the Creeps. All Rust code is located in the `rust` directory. You will need `cargo` installed in order to build the library and subsequently run the game.

If running on Windows, no changes will be needed after building. If running on Mac or Linux, you will need to update some paths in the `.gdnlib` file to point at the correct file.

# Dodge the Creeps

This is a simple game where your character must move
and avoid the enemies for as long as possible.

This is a finished version of the game featured in the
["Your first game"](https://docs.godotengine.org/en/latest/getting_started/step_by_step/your_first_game.html)
tutorial in the documentation. For more details,
consider following the tutorial in the documentation.

Language: GDScript

Renderer: GLES 3 (particles are not available in GLES 2)

Note: There is a C# version available [here](https://github.com/godotengine/godot-demo-projects/tree/master/mono/dodge_the_creeps).

Check out this demo on the asset library: https://godotengine.org/asset-library/asset/515

## Screenshots

![GIF from the documentation](https://docs.godotengine.org/en/latest/_images/dodge_preview.gif)

## Copying

`art/House In a Forest Loop.ogg` Copyright &copy; 2012 [HorrorPen](https://opengameart.org/users/horrorpen), [CC-BY 3.0: Attribution](http://creativecommons.org/licenses/by/3.0/). Source: https://opengameart.org/content/loop-house-in-a-forest

Images are from "Abstract Platformer". Created in 2016 by kenney.nl, [CC0 1.0 Universal](http://creativecommons.org/publicdomain/zero/1.0/). Source: https://www.kenney.nl/assets/abstract-platformer

Font is "Xolonium". Copyright &copy; 2011-2016 Severin Meyer <sev.ch@web.de>, with Reserved Font Name Xolonium, SIL open font license version 1.1. Details are in `fonts/LICENSE.txt`.
