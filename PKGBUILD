# Maintainer: Philip Rodrigues <phil-at-phiroict-dot-co-dot-nz>
pkgname=yt-parallel
pkgver=0.5.11
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" yt-parallel
    rm -f "${pkgdir}/.crates.toml"
    rm -f "${pkgdir}/.crates2.json"
    sudo cp ${pkgdir}/bin/${pkgname} /usr/local/bin/${pkgname}
    ${pkgname} -V
}

pkgdesc="A way to run downloads from yt-dlp in parallel"
url="https://github.com/phiroict/$pkgname"
license=("GPL-3-or-later")
depends=('yt-dlp>=2023.11.16')
source=(git+$url#tag=v$pkgver)
b2sums=('SKIP')