# FL Save Convert
A Win32 GUI program which modifies save files from the game Freelancer (2003).

## System Dependencies
Your system will need the latest [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022) installed or you may see the following error when trying to run fl_convert.exe

"The code execution cannot proceed because VCRUNTIME140.dll was not found. Reinstalling the program may fix this problem."

## General Information
### Problem:
Players attempting to use previous saves generated on Freelancer v1.0 have run into issues progressing in the single player story when trying to run the older save on an updated (v1.1) copy of the game. Reportedly this is due to an internal change of mission numbers between game versions[^1].

The Freelancer HD Edition project has requested a GUI application be built to decipher and offer the user the option to fix the needed parameters within their save files. This will allow players to transition their existing saves to FL HDE (https://github.com/BC46/freelancer-hd-edition/).

### Solution:
This program provides the user with the following two options...

1. Convert a selected save.
2. Fix and/or convert a selected save.

Both options will perform a backup of the selected save file and then use the GENE cipher[^2], provided by The Starport, to decipher the save into plaintext. If a user chooses to only convert their save, a new plaintext version of their save will be generated. If they chose to fix their save, then the save will be converted to plaintext and modified to allow them to continue playing in v1.1 without any interruption.

### Screenshots
![image](https://user-images.githubusercontent.com/16562693/193679396-ce78ca58-4889-43cf-b4b8-05da5717efc9.png)
![image](https://user-images.githubusercontent.com/16562693/193679641-06b19b40-3451-45c8-b9c3-6970e27f9a59.png)
![image](https://user-images.githubusercontent.com/16562693/193679734-0f421fc9-dbba-407d-8b82-dfb09219d6de.png)

### Footnotes
[^1]: https://the-starport.net/freelancer/forum/viewtopic.php?post_id=20878#forumpost20878
[^2]: https://the-starport.net/modules/mediawiki/index.php/MDB:%2Afl
