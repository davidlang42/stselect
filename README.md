# stselect
A CLI tool to manage selective syncing with [Syncthing](https://syncthing.net/) using the stignore file.

## Usage
- `stselect [PATH]` open an interactive editor for which folders to sync (NOT YET IMPLEMENTED)
- `stselect list [PATH]` list the folders currently selected to sync
- `stselect include [PATH] [SUB_FOLDER]` enable syncing of SUB_FOLDER
- `stselect exclude [PATH] [SUB_FOLDER]` disable syncing of SUB_FOLDER
- `stselect all [PATH]` enable syncing of all sub folders
- `stselect none [PATH]` disable syncing of all sub folders

PATH must be a valid path to the root directory (absolute or relative to current directory) of the syncthing share, and must therefore contain the hidden `.stfolder` directory.

SUB_FOLDER must be the name or path of a sub directory of the supplied root directory.

In all instances, PATH can be omitted to use the current directory as PATH, or both PATH and SUB_FOLDER can be omitted to use the current directory as SUB_FOLDER and its parent as PATH.

## How it works

Using this tool will rewrite the `.stignore` file in the PATH supplied so that:
- the first line is a comment saying when the file was last modified by stselect
- any folders selected for syncing will have a line like `!/FOLDER`, meaning don't ignore this folder
- any folders not selected for syncing will have a line like `#!/FOLDER`, which is only a comment
- the last line will be `/*/**`, meaning ignore all files/directories within any sub folders (except those specifically marked not to ignore above)
- any existing entries (which don't match the above) are maintained in the order they were previously but at the top of the file, giving them first priority

This will cause Syncthing to:
- sync all files and sub folders in the root directory (so that you can see a full list of folders which are available to be sync'd)
- not sync files within those sub folders by default
- sync files within any sub folders which are selected to be sync'd
- put an empty file called `WARNING_SUB_FOLDER_NOT_SYNCED` in any sub folders which are not sync'd so there is a visible difference between a non-sync'd folder and an empty folder

## Risks
If the `.stignore` file is modified during execution of `stselect`, or the program malfunctions, your original changes to `.stignore` could be lost. To mitigate this, when the new `.stignore` file is written, the old one is first moved into the `.stversions` folder and renamed to include the date and time it was moved.

If files exist in a non-synced folder, the user could think they are being synced when in fact they are not. The `WARNING_SUB_FOLDER_NOT_SYNCED` file is added as a visual indication of this, so it is important that it is not deleted. In addition to this, whenever `stselect` performs an operation that modifies the `.stignore` file, it will move any files in non-synced folders into `.stversions` and tell the user it has done it. This avoids data loss because the files still exist in `.stversions` and will hopefully make the user realise that folder is not synced when they see the files are gone. If the `stselect list` command is run files will NOT be moved, but the user will be warned of their existance.

## Known limitations
When checking if a non-synced folder is empty (or has files which should be moved to `.stversions`), the existing lines of `.stignore` are not considered, so all files will be moved (or listed as needing moving) even if the files would otherwise be ignored by Syncthing anyway.

The existing lines of `.stignore` are also not taken into account when listing the sub folders of the root directory which are available for syncing, so you may see extra folders listed which wouldn't sync even if they were selected to. It is generally advisable to not have additional lines in `.stignore` which weren't written by `stselect`, however if they are used you should know exactly what you are doing and fully understand how [Syncthing's ignore file](https://docs.syncthing.net/users/ignoring.html) works.

Behaviour with symlinks is not tested and may or may not work correctly. If you plan to use symlinks with `stselect` I suggest thoroughly testing the outcomes and [raising a PR](https://github.com/davidlang42/stselect/pulls) to update this documentation and/or [raise issues](https://github.com/davidlang42/stselect/issues) and make code changes to fix them.
