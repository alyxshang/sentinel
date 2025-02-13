# SENTINEL :performing_arts: :sparkles:

![GitHub CI](https://github.com/alyxshang/sentinel/actions/workflows/rust.yml/badge.svg)

***An API service for monitoring your server. :performing_arts: :sparkles:***

## ABOUT :books:

This repository contains the source code for an API service that returns valuable information on different hardware and software aspects of the machine the service is running on. The primary platform this service is meant for is a GNU/Linux system running SystemD. This service can, however, also be used on other operating systems and platforms.

## INSTALLATION :inbox_tray:

***Sentinel*** can be installed in two ways: i) as a SystemD service on a GNU/Linux system or ii) by compiling ***Sentinel*** and installing the executable on your system as an indepent service.

### Installing ***Sentinel*** as a SystemD service

***Sentinel*** offers two ways of being installed as a SystemD service: i) as a Debian package or ii) by setting up the SystemD service manually. To take either of these two approaches, the following software and tools must be installed on your system:

- [`libpam0g-dev`](https://packages.ubuntu.com/oracular/libpam0g-dev)
- [`libclang-dev`](https://packages.ubuntu.com/oracular/libclang-dev)
- [Git](https://git-scm.org)
- [Rust](https://rust-lang.org)


- 1.) First, you will need ***Sentinel's*** source code. This can be achieved by running the following command from a terminal session:

```Bash
git clone https://github.com/alyxshang/sentinel.git --depth 1
```

- 2.) Change directory into the source code's root directory:

```Bash
cd sentinel
```

- 3.) If you are using Debian, you can now build the Debian package by executing this command:

```Bash
cd packaging && bash debian.sh
```

- 4.) If you are not on Debian, please execute the following command from a terminal session. This command will compile ***Sentinel*** from source:

```Bash
cargo build --release
```

- 5.) If you are a Debian user, you should have a file called `sentinel.deb` inside the `packaging` directory. This Debian package can be installed with the command `sudo dpkg -i ./packaging/sentinel.deb`.

- 6.) If you built ***Sentinel*** directly with the command `cargo build --release`, there should now be a file called `sentinel` in the `target/release` directory of this repository. This file must be moved to the following directory: `/usr/local/bin`.

- 7.) As a Debian user, you can now check whether the ***Sentinel*** service is running with the command `sudo systemctl status sentinel`. If you are not a Debian user, you must move the file called `sentinel.service` in the `packaging/systemd` directory of this repository to the directory `/lib/systemd/system` or the appropriate location for SystemD services for your platform. Assuming this has been completed successfully, you can now enable the ***Sentinel*** service with the command `sudo systemctl enable sentinel` and start the service with the command `sudo systemctl start sentinel`.

- 8.) You should now be able to visit [`http://localhost:8000/sentinel/info`](http://localhost:8000/sentinel/info) and view some information about your system.

### Installing ***Sentinel*** as an independent executable

This approach will entail downloading ***Sentinel's*** source code, compiling ***Sentinel*** from source, and moving the resulting executable to a location registered on your system's `$PATH` environment variable. To take this approach, the following software and tools must be installed on your system:

- [Git](https://git-scm.org)
- [Rust](https://rust-lang.org)

- 1.) First, you will need ***Sentinel's*** source code. This can be achieved by running the following command from a terminal session:

```Bash
git clone https://github.com/alyxshang/sentinel.git --depth 1
```

- 2.) Change directory into the source code's root directory:

```Bash
cd sentinel
```

- 3.) Please execute the following command from a terminal session. This command will compile ***Sentinel*** from source:

```Bash
cargo build --release
```

- 4.) There should now be an executable binary called `sentinel` in the directory `target/release` inside this repository.

- 5.) Copy this binary to a location that is registered on your system's `$PATH` environment variable and ***Sentinel*** should now be installed.

- 6.) If you open a terminal session and enter the command `sentinel`, you should start seeing some logging information.

- 7.) You can now visit [`http://localhost:8000/sentinel/info`](http://localhost:8000/sentinel/info) and view some information about your system.

## USAGE :hammer:

### Rust API documentation

To view this crate's API please clone this repository and run the command `cargo doc --open` from the repository's root.

### JSON API documentation

The JSON API has two API routes:

- 1.) The API route to check user account credentials:

```text
/sentinel/auth
```

- 2.) The API route to retrieve server hardware information and software information:

```text
/sentinel/info
```

#### Verifying user account credentials

The API route for validating user credentials accepts `GET` requests with a JSON payload
of the following shape:

```JSON
{
    "user": "johndoe",
    "password": "password1"
}
```

This API route will return a JSON object with a boolean describing whether the credentials could be verified or not and has the following shape if verification of the credentials was successful:

```JSON
{
    "status": true
}
```

If the verification of the credentials was not successful, the returned JSON object will have this shape:

```JSON
{
    "status": false
}
```

#### Obtaining system information

The second API route when called from a payload-less `GET` request, returns a JSON object of the following shape:

```JSON
{
    "uptime": {
        "hours": 1,
        "minutes": 31,
        "seconds": 32
    },
    "storage_info": [
        {
            "name": "Disk 1",
            "available": "233.47",
            "taken": "84.65",
            "free": "148.82"
        },
        {
            "name": "Disk 2",
            "available": "233.47",
            "taken": "84.65",
            "free": "148.82"
        },
    ],
    "ram_info": {
        "available": "9.66",
        "taken": "6.13",
        "free": "3.53"
    },
    "net_info": {
        "netin": "0.780",
        "netout": "0.069"
    },
    "os_name": "Debian GNU/Linux",
    "os_ver": "12.0"
}
```

This response returns info on uptime, available disks, available RAM, the amount of data sent and received, the operating system name, and the operating system version. All size-related measurements are given in Gigabytes.

## CHANGELOG :black_nib:

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Sentinel :performing_arts: :sparkles:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
