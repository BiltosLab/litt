## idk how we gonna do this but let's write some steps we can start with
1. first what the app should do is parse the dir/files (leaving the main .litt dir and any other hidden files aka starts with ".")
2. push the files we got to a list
3. init a database of some sort idk
4. SHA-1 all of the files we will import  
5. compare the files we got to the database one if we have a new file or anything make sure to take note
6. start opening the files in that list one by one and parsing them and adding the diffs when staging? idk
7. the database file should be able to reconstruct a file even if original files are corrupted/damaged or even deleted.
8. if we have diffs we are able to commit the changes to the database aka add the diffs to the database file/s.
9. these are geniunely just brainstorming / shit storming ideas/steps but im here to learn babybyy so idc
    



## features/goals?
1. init an empty repo
2. stage the changes(from what i understand its the step before commiting for real? )
3. commit the to the database 
4. gg 
5. add remote support (idk how but we will see)
6. add cloning support
7. .littignore file to ignore certain stuff 
8. 