# Features

A list of features that `cportage` should have or will need. This file also talks about the requirements for each feature.

## cportage (program structure)

the program itself will be constructed like layers of an onion e.g.,;

- the window manager (GUI) will contain a "packager"
- each "packager" will act as a simple bridge to the python portage API
	- this class will also have these properties
		- `C` to download packages and add USE flags
		- `R` to query packages (locally and from the repos)
		- `U` to update packages and change USE flags
		- `D` to delete packages and remove USE flags
		- `L` to list locally installed packages and USE flags
	- it should also
		- obtain how they were compiled (e.g., what use flags did they use?)
		- retrieve all valid use flags of a package
		- sort all packages by figuring out which packages were directly installed by the user
			- e.g., did the user manually install vim or one of its dependencies

this is done so that the window manager can be separated from the packager such that the window only displays packages (text) and
the packager will only deal with exporting a list of packages or lists or simply put, just packages.

> each major function of `cportage` should be its own file (that contains classes, functions, etc.).

