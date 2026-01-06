# Homebrew Formula for Prism
# To install: brew install --HEAD https://raw.githubusercontent.com/artlostintime/prism/main/prism.rb

class Prism < Formula
  desc "Psychology survey data processor with automated scoring and quality checks"
  homepage "https://github.com/artlostintime/prism"
  url "https://github.com/artlostintime/prism/archive/refs/tags/v0.8.6.tar.gz"
  sha256 "" # Will be calculated during release
  license "MIT"
  head "https://github.com/artlostintime/prism.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
    
    # Install shell completions
    generate_completions_from_executable(bin/"prism", "completion")
    
    # Install documentation
    doc.install "README.md"
    doc.install "CHANGELOG.md"
    doc.install Dir["docs/*.md"]
    
    # Install example files
    (share/"prism/examples").install Dir["examples/*"]
  end

  def caveats
    <<~EOS
      Example configurations are installed to:
        #{share}/prism/examples

      Documentation is available at:
        #{doc}

      Get started:
        prism generate --list-scales
        prism generate --scale PHQ-9 > config.toml
        prism process -i data.csv -c config.toml -o clean.csv

      For more information:
        prism --help
        man prism  (if installed)
    EOS
  end

  test do
    # Test version output
    assert_match version.to_s, shell_output("#{bin}/prism --version")
    
    # Test help output
    assert_match "Psychology survey data processor", shell_output("#{bin}/prism --help")
    
    # Test scale listing
    output = shell_output("#{bin}/prism generate --list-scales")
    assert_match "PHQ-9", output
    assert_match "GAD-7", output
    
    # Test config generation
    output = shell_output("#{bin}/prism generate --scale PHQ-9")
    assert_match "[scales.PHQ9]", output
    assert_match "items = [", output
    
    # Test validation (should fail with no file, but command should work)
    assert_match "Error", shell_output("#{bin}/prism validate -c nonexistent.toml", 1)
  end
end
