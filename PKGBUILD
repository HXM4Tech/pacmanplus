# Maintainer: Krzysztof Stefańczyk <krzys.stefanczyk@gmail.com>
pkgname=pacman+
pkgver=1.0.0
pkgrel=1
pkgdesc="AUR-extended pacman package manager"
url="https://github.com/HXM4Tech/pacmanplus"
backup=("etc/pacman+.conf")
arch=('i686' 'pentium4' 'x86_64' 'arm' 'armv7h' 'armv6h' 'aarch64')
license=('GPL-3.0-or-later')
makedepends=('cargo')
depends=('git' 'pacman' 'libalpm.so>=14')
optdepends=('bat: colored pkgbuild printing')

build () {
  cd "$startdir"

  if [[ $CARCH != x86_64 ]]; then
    export CARGO_PROFILE_RELEASE_LTO=off
  fi

  cargo build --release --target-dir target
}

package() {
  cd "$startdir"

  install -Dm755 target/release/pacmanplus "${pkgdir}/usr/bin/pacman+"
  install -Dm644 pacman+.conf "${pkgdir}/etc/pacman+.conf"

  install -Dm644 completions/bash "${pkgdir}/usr/share/bash-completion/completions/pacman+.bash"
  install -Dm644 completions/fish "${pkgdir}/usr/share/fish/vendor_completions.d/pacman+.fish"
  install -Dm644 completions/zsh "${pkgdir}/usr/share/zsh/site-functions/_pacman+"

  install -dm755 "${pkgdir}/var/cache/pacman+"
  install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
  install -Dm644 README.md "${pkgdir}/usr/share/doc/${pkgname}/README.md"
}
