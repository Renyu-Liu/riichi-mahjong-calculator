# Riichi Mahjong Scoring Calculator
_Created by Renyu (@Renyu-Liu), Thomas (@akakrabz), and Karan (@Karan-Annam) as the final project for [CS 128H](https://honors.cs128.org)_
## Introduction
Riichi Mahjong has a complicated mechanism of scoring calculation. People often find it hard to calculate points manually. That's why we have introduced Riichi Mahjong Scoring Calculator. This is a Rust library that calculates the score of a winning Riichi Mahjong hand. It receives a complete input of the winning hand and game state and returns a detailed score breakdown, guiding players to redistribute their points.
## Installation
### Prerequisites
* **[Rust](https://rust-lang.org/tools/install/)** (Required)

* **[Visual Studio](https://visualstudio.microsoft.com/downloads/?q=build+tools)** (Windows)

* **[XCode](https://developer.apple.com/xcode/)** (MacOS)

* **[Git](https://git-scm.com)** and **[VSCode](https://code.visualstudio.com)** (Alternative)
  
### Download & Run

* Download and extract the ZIP file from [GitHub repo](https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator) or  run `git clone https://github.com/Renyu-Liu/Riichi_Mahjong_Scoring_Calculator.git` in terminal to clone the repository.

* Run `cd .../Riichi_Mahjong_Scoring_Calculator(-main)` in terminal to enter program folder.

* Run `cargo run` to launch the program. It will take a long time to compile for the initial run.

## User Manual

### 1: Select Winning Hand
<img width="778" height="664" alt="Composition_Phase" src="https://github.com/user-attachments/assets/e9055b58-ed2f-4f56-9e53-eaa1e299ee75" />

Click the tile in Tile Pool to add the tile into Hand Preview. Click the tile in Hand Preview to remove the tile.

You have to select at least 14 tiles to continue to next phase. Click "Confirm Hand" to continue.

<img width="960" height="161" alt="image" src="https://github.com/user-attachments/assets/cdbeea51-0a19-4676-83d9-1231a4ce9ad3" />

Click "Modify Hand" to return to tile selecting phase.

### 2: Select Winning Tile

<img width="980" height="121" alt="image" src="https://github.com/user-attachments/assets/63df3ce6-ea19-4462-aded-dbd4c9092134" />

Click "Select" button under Winning Tile to select from your hand. You must select one winning tile to continue to next phase.

<img width="1003" height="323" alt="image" src="https://github.com/user-attachments/assets/634e6f75-232c-406e-8fa3-77ba80162350" />

Click the tile image to select the winning tile.

<img width="943" height="144" alt="image" src="https://github.com/user-attachments/assets/5783abfc-737e-4876-82b4-01e9fd059248" />

Click the image of the winning tile to modify.

### 3: Select Game Information

<img width="948" height="105" alt="image" src="https://github.com/user-attachments/assets/e1e6b611-d399-4701-a34c-57922900cfc4" />

Click "Add Pon/Chii/Kan" to add pon/chii/kan you made in your round. Click "Change" to modify the open meld. Click "Remove" to remove the open meld.

<img width="997" height="216" alt="image" src="https://github.com/user-attachments/assets/93efbd4b-9976-44d7-8cd0-1b253a0a7b1b" />

It will display all possible pon/chii/kan from your hand. Click the meld to select pon/chii/kan you made in your round. 

<img width="974" height="183" alt="image" src="https://github.com/user-attachments/assets/d7a58a7c-829c-4a32-8194-11d00b2468bc" />

Click the meld image to remove the selected open meld.

<img width="976" height="685" alt="image" src="https://github.com/user-attachments/assets/630e3a28-b0fd-4d6a-91c8-2c41223f6da6" />

You may check for seats, context-dependent yaku, and choose the number of honba and akadora in your round. Click "Add" to add a (ura)dora tile. Click the image of (ura)dora tile to remove it.

### 4: Calculate Final Scores

Scroll down and click "Calculate Score" button to view the final score breakdown.

<img width="1002" height="550" alt="image" src="https://github.com/user-attachments/assets/7735387c-4d12-4021-b814-8fc26149512f" />

The score breakdown includes the total points, fu/han points, and yaku detected. It also guides players to redistribute their points. It will show "No Yaku Found" if no yaku is detected. 

To familiarize yourself with yaku and scoring rules, you may click "Rules" button at top right corner to view them at any time.

## Technical Overview

The flowchart below shows the logic flow of Riichi Mahjong Scoring Calculator:

<img width="964" height="1039" alt="flowchart" src="https://github.com/user-attachments/assets/b5991162-1413-4a28-9ac5-72704ff056c4" />

* Frontend Logic: The program directly handles all possible input conflicts according to Riichi Mahjong rules and guides users to correct their inputs. It ensures the input that sent to backend is recognizable.

* Backend Logic: The program involves multiple decision routes in the backend to detect all yaku, regular and irregular. Based on Riichi Mahjong scoring rules, there must be at least 1 yaku to calculate the score. Key crossroads include the check for hand structure and type of yaku. The final result is calculated based on the number of yaku and fu/han points.

## Challenges

* Some rare yaku with complicated rules are not correctly detected. We are working on it.

* We found that this program could not run on Windows system due to configuration problems. We are working on it.

## Reference

Scoring calculation is based on [standard Riichi Mahjong scoring rules](https://riichi.wiki/Japanese_mahjong_scoring_rules).

Yaku checker is based on [standard Riichi Mahjong yaku lists](https://riichi.wiki/List_of_yaku).

Images of tiles are from [riichi-mahjong-tiles](https://github.com/FluffyStuff/riichi-mahjong-tiles).

Image of Riichi Mahjong scoring rule is from [scoring rules sheet](https://www.reddit.com/r/Mahjong/comments/l5b221/riichi_mahjong_cheat_sheet_1_page_pdf_or_images/).
