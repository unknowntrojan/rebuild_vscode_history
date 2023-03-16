# rebuild_vscode_history

This program, given a path to the local vscode history folder and a prefix (which will be removed from the final reconstructed path), will attempt to reconstruct all files tracked by the vscode local history system. It will go through all entries, find the newest one, get it's path, and write it into a folder, attempting to keep the original folder structure intact.

you will need the nightly compiler to compile this.

![Example](https://i.imgur.com/KEejPAC.png)
