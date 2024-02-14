# What is Litt
Litt is an attempt to create something similar to git<br>i wanted a name similar to git so i replaced "g" with the 1st letter of my name and added another "t" at the end.<br>
This project aims to produce a working VCS "Version Control System" that actually works and it aims to be small and simple enough to be educational so anyone can read the code and understand how a Version Control System works.

# Features/Goals:
1. Initalize an empty repo (Start tracking changes in that repo)
2. Staging area (User can add the changes to be added to the next commit)
3. Commit the changes
4. Rollback and reset to any commit
5. Clone and Host a repo (that will be done much later in the project after implementing everything else.)
6. Display diffs between current files and last commit
7. Stay under 2000 Lines of code to be more readable and also (less lines means less bugs).
8. Ability to migrate a repo from git to litt or vice versa (Either litt itself will have a function to do that or i'll write a bash script for it idk)
9. Branching/tags and merging branches.


# TODO:
1. First big TODO and an important one is to remove unwraps and do proper error handling. [after we have a functioning prototype]
2. (git add implementation) Compress the files to an object blob with the file's hash as the name, Take care of index file and make a proper staging area. [doing it rn]
3. (git commit implementation) still learning what it does exactly.
4. Windows issues on path stuff idk why eg (.littignore needs this to block target folder ".\target" but on unix "./target") [Fixed in [c62e591](https://github.com/BiltosLab/littr/commit/c62e5913322dee52a0f29eb4296948183d603e33).]