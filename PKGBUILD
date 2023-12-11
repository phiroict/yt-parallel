pkgname=yt-parallel
pkgver=0.5.10
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" yt-parallel
}
pkgdesc="A way to run downloads from yt-dlp in parallel"
url="https://github.com/phiroict/yt-parallel/archive/refs/tags/v0.5.10.tar.gz"
license="GPL-3-or-later"
depends=('yt-dlp>=2023.11.16')
