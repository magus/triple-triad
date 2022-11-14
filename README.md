# triple-triad · [![publish](https://github.com/magus/triple-triad/actions/workflows/publish.yml/badge.svg?branch=release)](https://github.com/magus/triple-triad/actions/workflows/publish.yml)

can we solve **[Triple Triad | FINAL FANTASY XIV](https://na.finalfantasyxiv.com/lodestone/playguide/contentsguide/goldsaucer/tripletriad/)**?

## todo

## progress

| Rule         | Game | UI   |
| :---         | ---: | ---: |
| roulette     | ✅   | ✅   |
| all_open     | ✅   | ✅   |
| three_open   | ✅   | ✅   |
| same         | ✅   | ✅   |
| sudden_death | ✅   | ✅   |
| plus         | ✅   | ✅   |
| random       | -    | -    |
| order        | ✅   | ✅   |
| chaos        | ✅   | ✅   |
| reverse      | ✅   | ✅   |
| fallen_ace   | ✅   | ✅   |
| ascension    | ✅   | ✅   |
| descension   | ✅   | ✅   |
| swap         | ✅   | ✅   |
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


## publish

> The release workflow by default is triggered by pushes on the "release" branch. The action automatically creates a tag and title for the GitHub release using the application version specified in `tauri.config.json`.

> https://tauri.app/v1/guides/building/cross-platform


```sh
yarn release
# git push origin master:release
```
