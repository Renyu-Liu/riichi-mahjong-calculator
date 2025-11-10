# CS-128H-Group-12-Project
Group member names and NetIDs:
@Thomas (tj29) @Karan (annam17) @Renyu Liu (renyul2) 
Project Introduction

This project is a scoring calculator for Riichi Mahjong. It will take a game's state as an input and calculate the points when a user wins the game. We have chosen to work on this project because Mahjong is a pretty interesting game with a lot of different qualifications that can affect score, so there's quite a bit to take account of.  
The scoring rules in Riichi Mahjong is complicated for players to calculate with their hands. So this project would help users to calculate the points when users have won a game. 


Technical Overview

This will include 4 modules:

Input stage: Users will input their winning hand (unorganized), winning tile, and game conditions. 

1: Special Yaku Checker: special Yaku (seven pairs, 13 orphans, etc) will be checked first. If detected, jump over the next 2 modules and calculate scores.

2: Raw Hand Organizer: If no special Yaku is detected, organize the hand into 4 melds and 1 pair. Figure out the wait type.

3: Yaku Checker: check for ordinary yaku. If no yaku is found, throw error "no yaku".

4: score calculator: calculate the final scores by Fu and Fan

Output stage: return the final score

Likely by Checkpoint 1: Write some gate states, some impl to verify the state is valid. Write some hands, verify we can detect an illegal hand or invalid win.

Likely by Checkpoint 2: Finish game states, finish hands, make progress on the implementation to find the 'best' possible hand without endless iteration. 

Possible Challenges

Writing implementations may get complex and involved. Hands might be hard to implement the 'wildcard' for suits and unique hands. Calculating the score will probably be complex if we don't use a lot of OOP magic.  
We could also create a UI, with a display of how scoring is calculated, but this might not happen due to time. 

References

Input: Users will type in: 

1: the winning hand: including 14-18 tiles, indicating the winning tile; 

2: indicate which tiles are dora; 

3: indicate the prevalent wind and seat wind; 

4: check true / false: trumo/ron, concealed hand/open hand, Riichi/Ippatsu

1: check and identify the type of yaku

2: calculate fu, han, to get the points

Output: the points users get from winning this game