class Rift < Formula
  desc "Tiling window manager for macOS with virtual workspaces"
  homepage "https://github.com/DeepanshuMishraa/rift"
  version "0.1.0"
  head "https://github.com/DeepanshuMishraa/rift.git", branch: "main"
  license "Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", ".", "--locked", "--root", libexec
    bin.install libexec / "bin/rift"
    bin.install libexec / "bin/rift-cli"
    pkgshare.install "rift.default.toml"

    signing_identity = "C8B568E7DF057F0128BB6ACE47CE6CF9032F0A97"
    system "codesign", "--force", "--timestamp=none", "--sign", signing_identity,
           "--identifier", "git.acsandmann.rift", "#{bin}/rift"
    system "codesign", "--force", "--timestamp=none", "--sign", signing_identity,
           "--identifier", "git.acsandmann.rift.cli", "#{bin}/rift-cli"
  end

  def caveats
    <<~EOS
      Rift requires Accessibility permissions to control windows.
      Grant permissions in System Settings > Privacy & Security > Accessibility.

      To copy the example configuration:
        mkdir -p ~/.config/rift && cp #{pkgshare}/rift.default.toml ~/.config/rift/config.toml

      This fork's current source is installed with:
        brew install --HEAD #{name}
    EOS
  end

  service do
    run opt_bin / "rift"
    environment_variables PATH: std_service_path_env, LANG: "en_US.UTF-8"
    keep_alive true
    process_type :interactive
  end
end
