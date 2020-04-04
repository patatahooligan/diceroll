# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# The following guidelines are specific to BZR, GIT, HG and SVN packages.
# Other VCS sources are not natively supported by makepkg yet.

# Maintainer: Your Name <youremail@domain.com>
_pkgname_base=diceroll
pkgname=${_pkgname_base}-git
pkgver=0.2.0.r2.0f08bbc
pkgrel=1
pkgdesc="CLI dice rolling program written in rust"
arch=(any)
url="https://github.com/patatahooligan/diceroll"
license=('AGPL')
depends=()
makedepends=(git rust)
source=(git+https://github.com/patatahooligan/diceroll.git)
md5sums=('SKIP')

pkgver() {
    cd "$srcdir/${_pkgname_base}"

    printf "%s" "$(git describe --long | sed 's/\([^-]*-\)g/r\1/;s/-/./g')"
}

build() {
    cd "$srcdir/${_pkgname_base}"
    cargo build --release
}

package() {
    cd "$srcdir/${_pkgname_base}"
    install -Dm 755 target/release/diceroll "$pkgdir/usr/bin/diceroll"
}
