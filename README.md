# Purpose
In modern javascript frameworks, we are forced to use custom html attributes to write whether integration or end to end tests. These arbitrary attributes remains in production if we forget. This application aims to clean them up during or after compile time ( depends on your framework, you pick ).

## What this does?

On given arguments such as;
- html attribute
- Folder
It will recursively open up your files, search for given html attribute, remove them and update the file like a butter.

## Not done yet
Do not tell me i did not say, currently accepts only file and the attribute to look for.

# Licence
The MIT License (MIT)

Copyright (c) 2023-present Omer Atagun

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
