## La-Mulana (Original version) Randomizer
====

[![Electron](https://img.shields.io/badge/powered%20by-Electron-blue.svg)](https://electronjs.org/)
[![Build Status](https://travis-ci.org/progre/lmorandomizer.svg?branch=master)](https://travis-ci.org/progre/lmorandomizer) ![License](https://img.shields.io/github/license/progre/lmorandomizer.svg) [![Version](https://img.shields.io/github/release/progre/lmorandomizer/all.svg)](https://github.com/progre/lmorandomizer/releases)

Randomizer for La-Mulana (Original version). Original game is found at [Internet Archive](https://archive.org/details/La-Mulana)

This tool was inspired by [La-Mulana Randomizer](https://github.com/thezerothcat/LaMulanaRandomizer/wiki).

<img width="750" src="window.png">

**[Latest version is here](https://github.com/solairflaire/lmorandomizer/releases)**

----

Information
----

This application randomizes items in the La-Mulana script.dat file. Even after randomization, you can finish the game (but it may contain bugs...).

If you feel stuck, see some tips below. They may help.

----
How to use
----

Place seed and La-Mulana install directory, select options, then push Apply.

*Automatic Grail Tablet Registration*: Makes it so you can just walk in front of a grail tablet and it is registered for *Holy Grail* use.

*↓ to save at Grail tablets*: Pressing down at a Grail Tablet takes you to Xelpud. Loading brings you back to the last place saved at.

*Starting Items*: Select which items you want to start with.

Backside ROM combo is *Antarctic Adventure* and *Comic Bakery*. One is under the Birth Seal in *Inferno Cavern*.
The other is under the weight dais that activates the grail warp elevators in *Twin Labyrinth*.

### Current restrictions

- Items in *True Shrine of the Mother* and *Night Surface* aren't replaced.
  This means whatever is in the *Death Seal* location does NOT carry over to *True Shrine of the Mother*. Get it before killing all 8 guardians.
- *Mini Doll*, *Pepper*, *Anchor*, and *Mulana Talisman* are still given by their respective NPCs.
- ROMs acquired from scanning are not changed.
- Item names in shops are mismatched with actual item.

### Constraints on the system

- Certain situations can cause a softlock. For example, going to *Dimensional Corridor* without *Bronze Mirror*.
- Shop restrictions:
  - Maps are sold out if any map is acquired. Maps are not necessary to beat the game.
  - Sacred Orbs have the same problem as maps but have a real impact on game play.
  - Main Weapons severely break the shop interface if placed there.
  - Shuriken, Throwing Knives, Flares, Spears, and Bombs are always ammo in shops.
  - Silver Shield and Angel Shield don't sell out when bought. You do get the proper item if purchased.

### Stuck?

  - The *Talisman*, *Diary*, and *Treasure* are still there even if they are overwritten by a different item on the inventory screen.
  - The top right section of *Spring in the Sky* is accessible without the helmet. Use the *Feather* and *Grapple Claw* to get on the elevator.
  - The bombable wall in *Graveyard of the Giants* can be opened from the left side if you have *Bombs* and enough health.
  - Palenque's section of *Chamber of Extinction* can be accessed through the Gate of Time. Climb up the ladder and you get warped there.
  - The door in *Tower of Ruin* leading to Viy can be accessed by using *Bombs* instead of *Spears*. Hold down to roll bombs.
