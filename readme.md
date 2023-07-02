# yt-parallel

An utility to run yt-dlp processes in parallel. There is little tool specific code here, just run 
these downloads in parallel. 

Note that there are a lot of local settings here that you need to adapt to your needs, check the 
code comments for this. 

## Stack 

- yt-dlp (the tool is in brew, and most linux distro package managers )
- macos or linux (tool expects the following apps to be present)
  - which
  - mv
  - yt-dlp
  - rust stable 1.69+ (stable)
  - make
Needs this file to be present in the folder the command is run 
  - a text file called `videolist.txt` with a line separated list of youtube video urls (right click on the clip you want to add and select `copy link` and then paste the link in the file on a new line)
## Usage 

First time run the `make init` task to install the cargo dependencies that the `make check` task is using. You need not do that again.
Check the makefile how to build and deploy the application. Change the paths to fit your system.

`make build` to build and test the stack 
`make deploy` to push it to a global place you can run it from the commandline

Run by 
`./yt-parallel`

