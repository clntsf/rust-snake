scr_path="/Applications/snake.app/contents/macos/snake"
rm $scr_path
curl -o $scr_path https://raw.githubusercontent.com/ctsf1/rust-snake/master/target/debug/snake
chmod +x $scr_path