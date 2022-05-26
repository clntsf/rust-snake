touch ~/.zprofile
echo '\nalias update-snake="~/scripts/update_curl.sh"' >> ~/.zprofile
mkdir ~/scripts
curl -o ~/scripts/update_curl.sh https://raw.githubusercontent.com/ctsf1/rust-snake/master/update_curl.sh
chmod +x ~/scripts/update_curl.sh
rm setup_curl.sh