# Riichi Mahjong Calculator
_Created by Renyu (@Renyu-Liu) and Thomas (@akakrabz) as the final project for [CS 128H](https://honors.cs128.org)_
## Introduction
Riichi Mahjong Calculator is a desktop application built with Rust that calculates Riichi Mahjong scores. It takes the winning hand composition and game information as input to provide a detailed breakdown of points and payment distribution.
## Installation
See [RUN.md](RUN.md).

## User Manual

To familiarize yourself with yaku and scoring rules, you may click "Rules" button at top right corner to view them at any time.

### Composition Phase

<img width="1011" height="619" alt="image" src="https://github.com/user-attachments/assets/bd1857ef-ec4e-490d-943d-30e5527edef6" />

Users can build their baisc winning hand in this phase.

Tiles below button "Confirm Hand" is Tile Pool. Tiles above the button is Tile Preview. Click the tile in Tile Pool to add the tile into Hand Preview. Click the tile in Hand Preview to remove the tile.

You must select at least 14 tiles to continue to next phase. Click "Confirm Hand" to continue.

### Definition Phase

<img width="972" height="691" alt="image" src="https://github.com/user-attachments/assets/e5fbff67-c818-4148-824a-6764d5d4daa8" />
<img width="966" height="692" alt="image" src="https://github.com/user-attachments/assets/bc80cf64-0192-4b8c-b174-3a97abc4685a" />

Users can continue to complete their winning hand in this phase.

Click "Modify" button to go back to the Composition Phase.

#### Winning Tile

Click "Select" button under Winning Tile to select from your hand. 

<img width="986" height="224" alt="image" src="https://github.com/user-attachments/assets/73ff331a-5c54-483c-bf46-76647637ac12" />

You must select a single winning tile to continue to next phase.

After you select one, click the tile under Winning Tile to modify.

#### Open Hand

Click "Add Pon/Chii/Kan" to add pon/chii/kan you made in your round. 

After clicking "Add Kan", you can select "Closed/Open/Added Kan" to specify it.

Available Pon/Chii/Kan will automatically generate based on your winning hand.

<img width="1010" height="237" alt="image" src="https://github.com/user-attachments/assets/34e71ae2-4129-43e2-9423-15c9fc9be2a6" />

#### Game Info

Scroll down. You can check for win type, honba, prevalent wind and seat wind.

#### Special Yaku

You may check for particular yaku that don't depend on your winning hand. 

Note that some yaku have prerequisites. You can't check them until you fulfill prerequisites. These prerequisites are:

(Menzen = No tiles in Open Hand except Closed Kan)
* **Riichi**: Menzen
* **Double (Riichi) & Ippatsu**: Riichi is checked
* **Tenhou**: Win Type is Tsumo & Seat Wind is East & Menzen
* **Chiihou**: Win Type is Tsumo & Seat Wind is NOT East & Menzen
* **Renhou**: Win Type is Ron & Seat Wind is NOT East & Menzen
* **Haitei & Rinshan**: Win Type is Tsumo
* **Houtei & Chankan**: Win Type is Ron

#### Dora

You can select and modify Dora and Ura Dora in the same way as selecting winning tile. 

Only when Riichi is checked will Ura Dora be enabled.

Only when your winning hand contains at least one 5-tile will Red Dora be enabled. The upper limit of Red Dora is determined by 5-tile composition in your winning hand.

Click "Calculate Score" button to move to next phase and check for your final score.

### Result Phase

<img width="963" height="691" alt="image" src="https://github.com/user-attachments/assets/9681e480-4835-4803-b954-aee9b0147203" />

The score breakdown includes the total points, fu/han points, and yaku detected. It also guides players to redistribute their points. It will show "No Yaku Found" if no yaku is detected. 

Click "Back" button to go back to the previous phase. Click "Start Over" to start over the program.

Other examples:

<img width="1001" height="513" alt="image" src="https://github.com/user-attachments/assets/24a75d16-e3e6-49d7-bb16-c74ad95f83a8" />

<img width="959" height="652" alt="image" src="https://github.com/user-attachments/assets/1629cfa2-8b2c-48d0-95ea-58eb876e6f5d" />

## Technical Overview
### Data Structure

<img width="752" height="939" alt="Untitled Diagram drawio" src="https://github.com/user-attachments/assets/215981e1-450e-4c43-9f83-af0fa1036e3d" />

* Tile works as the smallest unit and makes up all related objects.

* Winning Hand and Game Info is the user input.

* Score is displayed the output.

* The program receives Winning Hand and Game Info and returns Score.

### Work Flow

<img width="964" height="1039" alt="flowchart" src="https://github.com/user-attachments/assets/b5991162-1413-4a28-9ac5-72704ff056c4" />

* Frontend Logic: The program directly handles all possible input conflicts according to Riichi Mahjong rules and guides users to correct their inputs. It ensures the input that sent to backend is recognizable.

* Backend Logic: The program involves multiple decision routes in the backend to detect all yaku, regular and irregular. Based on Riichi Mahjong scoring rules, there must be at least 1 yaku to calculate the score. Key crossroads include the check for hand structure and type of yaku. The final result is calculated based on the number of yaku and fu/han points.

## Challenges

### Current Issues
* The launching speed of this app is slow due to the time to load images. We are working to improve the loading speed.
* It's hard for common users to install this app. We are working to create executable object or application file for convenient access.

### Future Improvement Plan
* Develop mobile version.
* Improve the UI layout aesthetics. 
* Design functions to store history of input hands. 
* Add light and dark theme. 
* Add multiple language support (English, Chinese, Japanese). 
* Add input hand file format support to save and read hand locally. 
* Add settings to choose different version of Mahjong scoring rules (e.g. Sichuan Mahjong, Shanghai Mahjong, etc.).

## References

### Rules

* Scoring calculation is based on [standard Riichi Mahjong scoring rules](https://riichi.wiki/Japanese_mahjong_scoring_rules).

* Yaku checker is based on [standard Riichi Mahjong yaku lists](https://riichi.wiki/List_of_yaku).

* The rule used by the program is **standard 4-player East**. Nukidora and kita yaku are not supported. 

* Red dora varies in different versions. We use **5-man, 5-pin, 5-pin, 5-sou** as the red dora.

* For any disputes about yaku we use in our program, please counsel [yaku variations](https://riichi.wiki/Yaku_variations#Chiitoitsu).

### Images

* Images of tiles are from [riichi-mahjong-tiles](https://github.com/FluffyStuff/riichi-mahjong-tiles).

* Image of Riichi Mahjong scoring rule is from [scoring rules sheet](https://www.reddit.com/r/Mahjong/comments/l5b221/riichi_mahjong_cheat_sheet_1_page_pdf_or_images/).

### Font 
* [Arimo](https://fonts.google.com/specimen/Arimo)

## Acknowledgment

The initial inpiration of this program is the video game [Mahjong Soul](https://mahjongsoul.yo-star.com) by Yostar. It is a popular and interesting digital Riichi Mahjong game, and we enjoy it. One advantage of digital Mahjong is that the system helps players calculate points, but in an actual mahjong game, players find it hard to calculate points by hand, due to complicated scoring rules. This is why we created this app. During the development stage, we studied calculation algorithms used by Mahjong Soul to build the foundation of this program. During testing stage, we took players' score screenshots they shared on social media as test cases and compared the result from our program with the exact scores from Mahjong Soul to improve the performance. Thanks to this great game, we can combine our interests with our dedicated commitment to this project.
