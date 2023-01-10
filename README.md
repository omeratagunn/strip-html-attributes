# Purpose
In modern javascript frameworks, we are forced to use custom html attributes to write whether integration or end to end tests. These arbitrary attributes remains in production if we forget. This application aims to clean them up during or after compile time ( depends on your framework, you pick ).

## What this does?

On given arguments such as;
- html attribute
- Folder
It will recursively open up your files, search for given html attribute, remove them and update the file like a butter.

## Not done yet
Do not tell me i did not say, currently accepts only file and the attribute to look for.
