# Riichi Mahjong Scoring Calculator
_Created by Thomas (@akakrabz), Karan (@Karan-Annam), and Renyu (@Renyu-Liu) as the final project for [CS 128H](https://honors.cs128.org)_
## Introduction
Riichi Mahjong has a complicated mechanism of scoring calculation. People often find it hard to calculate points manually. That's why we have introduced Riichi Mahjong Scoring Calculator. This is a Rust library that calculates the score of a winning Riichi Mahjong hand. It receives a complete input of the winning hand and game state and returns a detailed score breakdown, guiding players to redistribute their points.
## Installation
### Step 1: Install Rust (skip if already done)

Visit official Rust website [Rustup](https://rustup.rs) to download Rust. Follow instructions on the website to configure Rust locally. Run `rustc --version` in the terminal to check if Rust is installed successfully.

### Step 2: Download the program

* Option 1: Download and extract the ZIP file from [GitHub repo](https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator)

* Option 2: Clone the repository from [GitHub repo](https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator):
    
```bash
git clone https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator.git
```

Note: Make sure the the folder directly contains `Cargo.toml` file (.../Riichi_Mahjong_Scoring_Calculator/Cargo.toml). Do not move `Cargo.toml` to other folders.

### Step 3: Run the program

* Change to the program folder:
    
```bash
    cd .../Riichi_Mahjong_Scoring_Calculator
```

Note: If you renamed "Riichi_Mahjong_Scoring_Calculator", `cd` goes with your renamed folder name

* Run the program:

```bash
    cargo run
```

## User Manual

### 1: Selecting Tiles
<img width="778" height="664" alt="Composition_Phase" src="https://github.com/user-attachments/assets/e9055b58-ed2f-4f56-9e53-eaa1e299ee75" />

Click the tile in Tile Pool to add the tile into Hand Preview. Click the tile in Hand Preview to remove the tile.

You have to select at least 14 tiles to continue to next phase. Click "Confirm Hand" to continue.

### 2: Selecting Winning Tile

<img width="964" height="220" alt="image" src="https://github.com/user-attachments/assets/0b3bebbf-d852-455c-9d48-e074e73736c6" />

Click "Modify Hand" to return to tile selecting phase.

Click "Select" button under Winning Tile to select from your hand. You must select one winning tile to continue to next phase.

<img width="1003" height="323" alt="image" src="https://github.com/user-attachments/assets/634e6f75-232c-406e-8fa3-77ba80162350" />

Click the tile image to select the winning tile.

<img width="947" height="246" alt="select_win_tile_done" src="https://github.com/user-attachments/assets/a95fd1c4-01f5-4251-96cc-431cead03c6e" />

Click the image of the winning tile to modify.

### 3: Selecting Game Context

<img width="948" height="105" alt="image" src="https://github.com/user-attachments/assets/e1e6b611-d399-4701-a34c-57922900cfc4" />

Click "Add Pon/Chii/Kan" to add pon/chii/kan you made in your round. Click "Change" to modify the open meld. Click "Remove" to remove the open meld.

<img width="997" height="216" alt="image" src="https://github.com/user-attachments/assets/93efbd4b-9976-44d7-8cd0-1b253a0a7b1b" />

It will display all possible pon/chii/kan from your hand. Click the meld to select pon/chii/kan you made in your round. 

<img width="974" height="183" alt="image" src="https://github.com/user-attachments/assets/d7a58a7c-829c-4a32-8194-11d00b2468bc" />

Click the meld image to remove the selected open meld.

<img width="974" height="651" alt="image" src="https://github.com/user-attachments/assets/014c7538-7acf-493e-a489-58666e30e16a" />

You may check for seats, context-dependent yaku, and choose the number of honba and akadora in your round. Click "Add" to add a (ura)dora tile. Click the image of (ura)dora tile to remove it.

### 4: Generating Score Breakdown

Scroll down and click "Calculate Score" button to view the final score breakdown.

<img width="1008" height="551" alt="image" src="https://github.com/user-attachments/assets/ebe2d444-c1a8-46e9-bea5-243fab2cdb55" />

The score breakdown includes the total points, fu/han points, and yaku detected. It also guides players to redistribute their points.

## Technical Overview

The flowchart below shows the logic flow of Riichi Mahjong Scoring Calculator:

<img width="964" height="1039" alt="flowchart" src="https://github.com/user-attachments/assets/b5991162-1413-4a28-9ac5-72704ff056c4" />

* Frontend Logic: The program directly handles all possible input conflicts according to Riichi Mahjong rules and guides users to correct their inputs. It ensures the input that sent to backend is recognizable.

* Backend Logic: The program involves multiple decision routes in the backend to detect all yaku, regular and irregular. Based on Riichi Mahjong scoring rules, there must be at least 1 yaku to calculate the score. Key crossroads include the check for hand structure and type of yaku. The final result is calculated based on the number of yaku and fu/han points.

## Challenges

* Loading speed: We found that the loading time for a large amount of tiles is too long, which is restricted by the image loading speed. We are working on it.

* Yaku check: Some rare yaku with complicated rules are not correctly detected. We are working on it.

## Reference

Scoring calculation is based on [standard Riichi Mahjong scoring rules](https://riichi.wiki/Japanese_mahjong_scoring_rules).

Yaku checker is based on [standard Riichi Mahjong yaku lists](https://riichi.wiki/List_of_yaku).

Images of tiles are from [riichi-mahjong-tiles](https://github.com/FluffyStuff/riichi-mahjong-tiles).
