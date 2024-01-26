# Maintainer: Philip Rodrigues<phil@phiroict.co.nz>
pkgname=yt-parallel
pkgver=0.5.19
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="This application runs download tools (default is yt-dlp) in parallel to download batches of videos."
license=('GPL-2.0-or-later')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/phiroict/yt-parallel
}
