appdir="/Applications/snake.app/contents/resources/update_curl.sh"
touch ~/.zprofile; echo "\n"alias update-snake=\"$appdir\" >> ~/.zprofile
curl -o $appdir https://raw.githubusercontent.com/ctsf1/rust-snake/master/update_curl.sh; chmod +x $appdir
rm setup.sh