# triple-triad
can we solve **[Triple Triad | FINAL FANTASY XIV](https://na.finalfantasyxiv.com/lodestone/playguide/contentsguide/goldsaucer/tripletriad/)**?


## todo

- all_open plan
  - `start` command should capture `setup_game` AND kickoff `pre_game` command
  - `pre_game` command should handle checking a new `app_state.setup.rules` field
    - handle setting the correct `app_state.status`
    - e.g. for `all_open` rule
      - `game.rules.all_open` -> set `app_state.status` to `'all_open'`
      - client reads `'all_open'` status and adjusts card callbacks
      - client state use a `Set` to track selected cards, restrict maximum to `5` cards
      - clicking cards toggles their selection state (use `opacity` to show current selection)
      - when 5 cards are selected, display a `Confirm` button in the `Status`
      - `Confirm` button calls `game_command('all_open', { cards })` with the vector of card indices
    - finishes when all `game.rules` are marked in `setup.rules`
    - set `app_state.status` to `'turns'`
  - when `status` is `'turns'` allow main game flow, alternating `execute_turn` commands


| Rule         | Game | UI   |
| :---         | ---: | ---: |
| roulette     | -    | ❌   |
| all_open     | ✅   | ✅   |
| three_open   | ✅   | ✅   |
| same         | ✅   | ✅   |
| sudden_death | ✅   | ✅   |
| plus         | ✅   | ✅   |
| random       | ✅   | ❌   |
| order        | ✅   | ❌   |
| chaos        | ✅   | ✅   |
| reverse      | ❌   | ❌   |
| fallen_ace   | ❌   | ❌   |
| ascension    | ❌   | ❌   |
| descension   | ❌   | ❌   |
| swap         | ✅   | ❌   |
| draft        | -    | -    |

> **[Find NPCs for testing different rules](https://arrtripletriad.com/en/npcs)**

- [setup isolation pattern for IPC between tauri backend and next frontend](https://tauri.app/v1/references/architecture/inter-process-communication/isolation)


## run

> **Run tauri app via src-tauri/src/main.rs**
```sh
yarn tauri dev --release
```


> **Build app for distribution (app, dmg, etc.)**

> https://tauri.app/v1/guides/distribution/publishing/

```sh
cargo tauri build
```
