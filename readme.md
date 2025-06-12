 # yt-parallel
yt-dlp is a video file downloader, it has been designed to download one file at the time. It is quick enough but if you have a set of downloads it is faster 
when running in parallel. Now this is use-case that works for me, so your milage may vary, it is also a nice Rust exercise. You list a list of video urls in a textfile and then download it as a batch. It will place it in a folder that is the current date for instance 20240101 and at the end it moves it to a location that could be on a SAN/NAS so then it is shared.  
In short: 
An utility to run yt-dlp processes in parallel. There is little tool specific code here, just run 
these downloads in parallel. 
It is possible to use another download tool by passing it in the commandline for instance: `yt-parallel --video-download-tool your-tool-here`
Note that there are a lot of local settings here that you need to adapt to your needs, check the 
code comments for this. There are a number of arguments you can pass. 
Now this was a study project for me to learn Rust, so it is not a full blown application, but it works for me.
Note that this bash script does the same: 

```bash
#!/usr/bin/env bash
set -euo pipefail
TARGET=/run/media/phiro/huge/media/youtube
LIST="$HOME/Desktop/videolist.txt"
YTDLP_OPTS=(
  --sponsorblock-remove default
  --fragment-retries infinite
  --buffer-size 16K
)
FOLDER=$(date +%Y%m%d)
mkdir -p "${TARGET}/${FOLDER}" || exit 2
cd "${TARGET}/${FOLDER}" || exit 1
# Read each URL, fire off yt-dlp in the background, silencing stdout+stderr
while IFS= read -r url; do
  yt-dlp "${YTDLP_OPTS[@]}" "$url" > /dev/null 2>&1 &
done < "$LIST"

# Wait for all background jobs to finish
wait

cd -
```
So, yeah, overengineered for a simple task, but it is a learning exercise.

## Stack 

### Mac and Linux
- yt-dlp (the tool is in brew, and most linux distro package managers )
- macos or linux (tool expects the following apps to be present)
  - which
  - mv
  - yt-dlp
  - rust stable 1.69+ (2021 edition support)
  - make
### Windows
  - yt-dlp
  - Visual Studio C compiler (Just use the VS installer and install the C++ tools) If you install rust from the installer it will guide you to the C++ compiler you need to install. 
  - rust stable 1.69+  (2021 edition support)
  - python 3.10+ 
  - Recommended is to install the tools with `winget` or `choco` just for convenience. 
Needs this file to be present in the folder the command is run 
  - a text file called `videolist.txt` with a line separated list of youtube video urls (right click on the clip you want to add and select `copy link` and then paste the link in the file on a new line)
## Usage 

First time run the `make init` task to install the cargo dependencies that the `make check` task is using. You need not do that again.
Check the makefile how to build and deploy the application. Change the paths to fit your system.

If you are on the Mac and want to create the container stack, run `make init_arm` first to install the rust
compilers and linkers for linux amd first. 

`make build` to build and test the stack 
`make deploy` to push it to a global place you can run it from the command-line, this is for linux and macos. It will deploy to a location the path would find it. By tradition this is `/usr/local/bin/`
`make deploy_win` to build the windows executable and copies it to a place in the path, in this case `i:\Apps` but you would need to set one location up and add that to the %PATH%
 
Usage
```text
Usage: yt-parallel [OPTIONS]

Options:
  -l, --location-video-list <LOCATION_VIDEO_LIST>
          Location of the videolist.txt file [default: ./videolist.txt]
  -v, --video-download-tool <VIDEO_DOWNLOAD_TOOL>
          [default: yt-dlp]
  -d, --debug-level <DEBUG_LEVEL>
          [default: info] [possible values: trace, debug, info, warn, error]
  -m, --move-target <MOVE_TARGET>
          [default: ]
  -h, --help
          Print help
  -V, --version
          Print version

```

