rm -rf $HOME/.local/sdl2

mkdir -p $HOME/.local/srcs/
cd $HOME/.local/srcs

apt-get download libsdl2-ttf-2.0-0
apt-get download libsdl2-ttf-dev
apt-get download libsdl2-image-2.0-0
apt-get download libsdl2-image-dev
dpkg -x libsdl2-ttf-2.0-0*.deb $HOME/.local/sdl2
dpkg -x libsdl2-ttf-dev*.deb $HOME/.local/sdl2
dpkg -x libsdl2-image-2.0-0*.deb $HOME/.local/sdl2
dpkg -x libsdl2-image-dev*.deb $HOME/.local/sdl2

mv $HOME/.local/sdl2/usr/* $HOME/.local/sdl2

rm -rf $HOME/.local/sdl2/usr

echo 'export LIBRARY_PATH="$HOME/.local/sdl2/lib/x86_64-linux-gnu:$LIBRARY_PATH"' >> $HOME/.bashrc
