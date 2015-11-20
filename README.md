# Locomotive :steam_locomotive:
## Spread your Steam games locally

Locomotive is a network P2P game sharing tool.
The idea is simple: using the large LAN bandwidth to install and update your Steam games.

Over the years, I found myself multiple times in a situation where we would gather at a friend's place, but found ourselves stuck at installing a game to play together. Alternatives exists, but I thrive for something simpler:
- Copying the directories implies a network share
- Packing/Unpacking from Steam is long and uses a lot of space

Games are getting updated at a relentless pace, some more than 3 times/week. I would like to use an updated local version to pull in updates.

Here are the feature set I'm looking to implement:
- Autodiscovery of other running Locomotive
- Exposition of currently installed Steam games
- Install from a Locomotive
- Update from a Locomotive
- CLI interface

Things I'm still considering:
- Using a SteamDB-like API to be Steam format aware, i.e. enable sharing of cross-platform games. A Linux user _could_ install the common parts of a certain games from a Locomotive running on Windows or OSX. A lot of times, models/textures/sounds etc. are available as a separate bundle, so that could be shared.
- Implement P2P transfer, to improve speed and reliability
- Implement a GUI
- PKI security

## Licence
MIT

## Authors
Micaël Bergeron <micaelbergeron@gmail.com>



