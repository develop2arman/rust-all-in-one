VAGRANTFILE_VERSION = "2"

$provision = <<SCRIPT
sudo apt update
sudo apt install -y curl

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.30
cd /vagrant
cargo build
SCRIPT


Vagrant.configure(VAGRANTFILE_VERSION) do |config|
    config.vm.box = "bento/ubuntu-20.10-amd64"
    config.vm.post_up_message = "Hope that you're enjoying Rust all in one!\n-TS ArmanRiazi"
    config.vm.provision "shell", inline: $provision
end
