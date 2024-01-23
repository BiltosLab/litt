# What is Litt
Litt is an attempt to create something similar to git , i wanted a name similar to git and i couldnt waste time on a name so i replaced g with the first letter of my name and added a t and the end.
"Litt" can mean anything depending on your mood.

This project aims to produce a working VCS "Version Control System" that actually works and it aims to be small and simple enough so anyone can read the code and understand how a VCS works while also being educational.

# Main features(Goals)
1. Initalize an empty repo (Start tracking changes in that repo)
2. Staging area (User can add the changes to be added to the next commit)
3. Commit the changes
4. Rollback and reset to any commit
5. Clone and Host a repo (that will be done much later in the project after implementing everything else.)
6. Display diffs between current files and last commit
7. Stay under a certain number of LOC (less lines means less bugs and it becomes easier to understand due to simplicity) Number TBD.
8. Ability to migrate a repo from git to litt or vice versa (Either litt itself will have a function to do that or i'll write a bash script for it idk)


# TODO:
1. First big TODO and an important one is to remove unwraps and do proper error handling.
2. (git add implementation) Compress the files to an object blob with the file's hash as the name. [doing it rn]
3. (git commit implementation) still learning what it does exactly.
