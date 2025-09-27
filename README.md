# Musiq Updater
On each run of this program the Musiq installation of this machine will be
updated and ran in a tmux.
## Usage
Set `MUSIQ_SOURCE_PATH` and `MUSIQ_PATH` to their desired values and run this program.
Any other environment variables will be inherited by the compilation process and Musiq itself.\
If these are not specified, they can be supplied via command-line arguments in the order
of ``musiq-updater <MUSIQ_SOURCE_PATH> <MUSIQ_PATH>``