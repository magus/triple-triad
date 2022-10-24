# data

extract and parse game data, textures and images from ffxiv files

## run

> **Generate game data json files consumed by triple-triad rust program**
```sh
yarn build:data
```

> **Generate spritesheet of all card images**
```sh
yarn build:spritesheet
```

> **Test the card image creation with a specific card and permuting params**
```sh
yarn test:card
```

## setup `data/game`

- On **FFXIV machine**
  - [Install FFXIV TexTools](https://github.com/TexTools/FFXIV_TexTools_UI)
    - Search for **triad**
    - Select **CardTripleTriad**
    - Toggle **HD Texture**
    - Path to the texture **`ui/uld/cardtripletriad_hr1.tex`** at bottom
    - Copied into `extract-ui-data` script to extract as image
  - [Download the latest version of SaintCoinach.Cmd from the releases page](https://github.com/ufx/SaintCoinach/releases)
    - Unzip the `SaintCoinach.Cmd` archive and navigate into it
    - Copy paste `game/1-extract-ui-data.bat` into the `SaintCoinach` folder
    - Double-click to run `1-extract-ui-data.bat`
    - Open the new game version folder which contains `data.zip`
    - Send `data.zip` to **development machine**
- On **development machine**,
  - Ensure `data.zip` is inside `data/game` folder i.e. `data/game/data.zip`

## data

https://github.com/xivapi/ffxiv-datamining

> **card id and stats**
> ./game/rawexd/TripleTriadCardResident.csv

> **card names by id**
> ./game/rawexd/TripleTriadCard.csv

> **card types and names**
> ./game/rawexd/TripleTriadCardType.csv

> **card npcs id map to guaranteed cards etc.**
> ./game/rawexd/TripleTriad.csv
> **lookup npc id by card npc id**
> ./game/rawexd/ENpcBase.csv
> **lookup card npc name by id**
> ./game/rawexd/ENpcResident.csv

> **rules**
> ./game/rawexd/TripleTriadRule.csv



