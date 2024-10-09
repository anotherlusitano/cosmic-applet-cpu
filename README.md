<div align="center">
  <h1>CPU applet for COSMIC Desktop</h1>
  <br>
  <div>
    <img alt="chat" src="https://github.com/user-attachments/assets/271b2a14-8958-4b04-b187-edd2a75c00c7" width="280">
  </div>
</div>

## Installing this applet

To install this COSMIC applet, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install this applet:

```sh
just build-release
sudo just install
```

## Configuring this applet

To change the time this applet will refresh the cpu usage, you need to create the file `refresh_time` and insert the seconds into the file.
