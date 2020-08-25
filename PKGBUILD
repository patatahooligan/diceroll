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
    gzip -k man/diceroll.1

    install -Dm 755 target/release/diceroll "$pkgdir/usr/bin/diceroll"
    install -Dm 755 man/diceroll.1.gz "$pkgdir/usr/share/man/man1/diceroll.1.gz"
}
