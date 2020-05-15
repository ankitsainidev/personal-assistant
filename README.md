# Simple toolbox
    This is like a simple assistant.
    Supports functionality for common tasks such as notes, todos, timer, weather etc.


    Download the release file and add it to path.
    Start by using `pat` in command-line

    It depends to two files, a sqlite3 database and an notification sound. Both are created
    by the script itself.

``` 
Usage:  
    notes:  
        `pat note "A quote can make your day"`  to create new note   
        `pat note all`  to get list of all the notes  
        `pat note delete id`  to delete note(s)  

        NOTE: id of note is displayed in the output by `pat note all`  

    weather:   
        `pat weather`  for current weather  

    quote:  
        `pat quote`  to see an inspirational quote  

    save:  
        Useful in saving contact numbers, address, names etc.  
        syntax: `pat save {key} {value}`  
        `pat save "phone number" "+1111"`  to create new save  
        `pat save "phone number"`  to get the saved value  
        `pat save delete "phone number"`  to remove the saved value  
        `pat save all`  to get list of saved pairs  

        Future work  
            `pat save copy "phone number"`  to copy value to clipboard  

        NOTE: keys should be unique for different saves and  
            

    todos:  
        `pat todo "Pay electricity bills"`  for new todo  
        `pat todo done id`  to mark todo(s) as completed  
        `pat todo clean`  to clear all completed todos  
        `pat todo all`  to get list of all the todos  

        NOTE: To get id of todo use `pat todo all`  

    help:  
        `pat` to reach this introduction  
```
To reset just remove $HOME/.pat directory.
