# Sliver extension -  uac bypass via cmstp 

Sliver extension for bypassing UAC via cmstp written in Rust.

## Installation

Compile the DLL:
```shell
cargo build --release --lib --target x86_64-pc-windows-gnu
cp .\target\x86_64-pc-windows-gnu\release\uac_bypass_cmstp.dll .
```

Install the extention in sliver 

```shell
extensions install /path/to/sliver_extention_uac_bypass_cmstp
extensions load /path/to/sliver_extention_uac_bypass_cmstp
```

## Usage

```
sliver> uac_bypass_cmstp <path_to_exe>
```


## Resources: 
- Sliver extension template https://github.com/Paradoxis/Sliver-Rust-Extension-Template
- UAC bypass code https://medium.com/@harikrishnanp006/uac-bypass-on-windows-abe21d74f050 
