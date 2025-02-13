#!/usr/bin/env bash
# Maintainer: Alyx Shang

NAME="sentinel"
VERSION="0.1.0"
ARCH="amd64"
AUTHOR="Alyx Shang"
DESCRIPTION="An API service for monitoring your server."

# Creating the needed directories.
mkdir $NAME
mkdir $NAME/usr
mkdir $NAME/lib
mkdir $NAME/usr/local
mkdir $NAME/lib/systemd
mkdir $NAME/usr/local/bin
mkdir $NAME/lib/systemd/system

# Building sentinel and copying files.
cd ..
cargo build --release
cd packaging
mv ../target/release/$NAME ./$NAME/usr/local/bin
cp -f systemd/sentinel.service ./$NAME/lib/systemd/system

# Creating and writing the control file.
mkdir $NAME/DEBIAN
touch $NAME/DEBIAN/control
echo "Package: $NAME" >> $NAME/DEBIAN/control
echo "Version: $VERSION" >> $NAME/DEBIAN/control
echo "Maintainer: $AUTHOR" >> $NAME/DEBIAN/control
echo "Architecture: $ARCH" >> $NAME/DEBIAN/control
echo "Description: $DESCRIPTION" >> $NAME/DEBIAN/control

# Creating and writing the post-installation script.
touch $NAME/DEBIAN/postinst
chmod a+x $NAME/DEBIAN/postinst
echo "#!/usr/bin/env bash" >> $NAME/DEBIAN/postinst
echo "systemctl enable sentinel" >> $NAME/DEBIAN/postinst
echo "systemctl start sentinel" >> $NAME/DEBIAN/postinst
echo "echo \"The sentinel service is installed and enabled.\"" >> $NAME/DEBIAN/postinst

# Build the package.
dpkg-deb --build $NAME