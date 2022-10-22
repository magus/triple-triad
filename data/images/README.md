# images

## todo

- compose sprite sheet of transparent card backgrounds (remove card background from rake)
- in our UI we can compose the transparent cards over the correct background to create red/blue cards


## setup

> Instructions derived from repo below
> https://github.com/mattantonelli/xiv-data#images

- On **FFXIV machine**
  - [Install FFXIV TexTools](https://github.com/TexTools/FFXIV_TexTools_UI)
    - Search for **triad**
    - Select **CardTripleTriad**
    - Toggle **HD Texture**
    - Path to the texture **`ui/uld/cardtripletriad_hr1.tex`** at bottom
    - Copied into `extract-ui-data` script to extract as image
  - [Download the latest version of SaintCoinach.Cmd from the releases page](https://github.com/ufx/SaintCoinach/releases)
    - Unzip the `SaintCoinach.Cmd` archive and navigate into it
    - Copy paste `extract-ui-data.bat` into the `SaintCoinach` folder
    - Run `extract-ui-data.bat`
    - Open the new game version folder which contains `ui.zip`
    - Send `ui.zip` to **development machine**
- On **development machine**,
  - Ensure `ui.zip` is in `~/Downloads`
  - Run `extract-images-from-ui.sh`
  - Run `generate-card-images.js`

