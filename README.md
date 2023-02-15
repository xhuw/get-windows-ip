# get windows ip

Tool to get the host address of your windows machine from WSL2. The normally recommended solution is to 
read the address of the nameserver from the auto generated /etc/resolv.conf however you may have elected
to use a custom dns resolver rather than using windows. in which case the contents of this file will be 
useless. Instead run ipconfig.exe from wsl and parse the output.

```
cargo run

## or

cargo install --path .
get-windows-ip
```
