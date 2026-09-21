# MARIONETTE

This file explains how puppet expects commands. You as a developer who creates scripts or programs to control puppet running devices can use this document for reference.

# Introduction

Instructions are JSON data trasmitted as UTF-8 string via UDP broadcasts. Format is strictly evaluated. Messages can be rejected if format fails, if that didn't happen, you're quite lucky.

## Fields

```txt
{
  "header": "!Puppet93",
  "token": "MeowMeowMeow",
  "device_id": "0",
  "msg_type": "AUD",
  "msg_data": "https://yumicoradio.net/stream",
  "msg_params": [
    "30s"
  ],
  "timestamp": 1785863627
}
```

### header

Denotes a group. A base level check made by puppet, header has to match the value by the same name configured in puppet. This can be used to divide hosts into groups

### token

A simple string to block intruders (TODO: Token is not hashed as of writing this). This is purely intended for security.

### device_id

ID that identifies the puppet host. Is a string. If incoming message has this field as `*`, all puppet clients irrespective of ID are expected to follow the instruction.

### msg_type

Classifies instruction types. Explained in brief by this document.

- VOD - Void
- TXT - Toast
- AUD - Audio
- IMG - Image
- VID - Video

### msg_data

Data for the action to process. Can be a URL, message or anything that pertains to the instruction type.

### msg_params

Extra parameters for the commands. ex: "{"time": "50s"}" for AUD to play for 50s. This is something that is specific to the intruction type. Puppet does not impose strict checking on this field, ie. instructions won't be ignored even if the parameters mentioned are wrong. This will however indeed lead to actions caused by the same (for instance, wrongly formatted parameter to play the video for 50 seconds will not affect the default behvaiour of the video being played till the end).

### timestamp

UNIX EPOCH for puppet to instruct when to start the action (good for precision). If the value is 0, puppet is intructed to perform the action ASAP.

Do note that some instructions (like playing a heavy video) might take time by itself to load, setting an earlier timestamp does not inherently solve this problem. If you have ideas to solve this dilemma, do let us know - PRs are welcome.

# Instructions - msg_type

`msg_type` is a string that denotes the type of command.

## VOD

Void. This instructs puppet to show a blank screen. Useful for clearing display related instructions.

Unfortunately by design, `msg_data` is deemed compulsory -> omitting it would break parser (TODO: Fix this) -> hence `msg_data` must be present in field.

### Example(s)

Display void indefinitely

```bash
echo '{"header": "!Puppet93", "token": "MeowMeowMeow", "device_id": "0", "msg_type": "VOD", "msg_data": "", "timestamp": 0}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

## TXT

`TXT` as of writing this document is Toast. Initially, it only logged the message received on cli - this is useless now as puppet is now meant to be launched full screen. Toast is a small dialog displayed as an in-app notification within all modes. This means `TXT` messages can be shown despite the current mode the screen is on.

### Data - msg_data

Message to be displayed as toast

```txt
Meow
You See What You Think You See
Do you think the desire to save oneself is enough of a justification to try and save the world?
```

### Parameters - msg_params

#### type

Since the [toast library we use](<>) supports error, warning and info toast types, we implemented `type` to reflect the same.

- `warning`: denotes messages that need attention
- `critical`: denotes messages that need immediate attention
- `info`: denotes informatic messages

Default `type` is `info`.

### Example(s)

Toast `Meow` as `info`.

```bash
echo '{"header":"!Puppet93","token":"MeowMeowMeow","device_id":"*","msg_type":"TXT","msg_data":"Meow","timestamp":0}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

Toast `Meow` as `critical`.

```bash
echo '{"header":"!Puppet93","token":"MeowMeowMeow","device_id":"*","msg_type":"TXT","msg_data":"逃げるんだよ!","timestamp":0, "msg_params": {"type": "critical"}}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

## IMG

Loads an image and shows it on the screen.

### Data - msg_data

URL that points to local or remote file

```txt
file:///opt/puppet/grin.png
https://cdn.dawn.org.in/blog/penguin/posts/20250116/HashToolDialog.avif
```

### Example(s)

Display [raven dotfile screenshot](https://cdn.dawn.org.in/blog/penguin/posts/20250116/hyprdot-2.avif) indefinitely

```bash
echo '{"header": "!Puppet93", "token": "MeowMeowMeow", "device_id": "0", "msg_type": "IMG", "msg_data": "https://cdn.dawn.org.in/blog/penguin/posts/20250116/hyprdot-2.avif", "timestamp": 0}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

## AUD

Plays an audio in the background. This is a non-display instruction - meaning it can still keep running and not get blocked by other instructions.

### Data - msg_data

URL that points to local or remote file

```txt
file:///opt/puppet/liyue.m4a
https://samplelib.com/mp4/sample-5s.mp4
```

### Parameters - msg_params

#### time

String that denotes humanized time.

```
50s - 50 seconds
1h - 1 hour
```

Audio won't loop if time exceeds audio length.

> [!WARNING]
> Omitting this parameter will play the audio till end. This means indefinit for continous streams like internet radio.
> Do note that there is no way as of writing this to kill running instructions with commands

### Example(s)

Play [yumicoradio.net](https://yumicoradio.net) for 1 hour

```bash
echo '{"header": "!Puppet93", "token": "MeowMeowMeow", "device_id": "0", "msg_type": "AUD", "msg_data": "https://yumicoradio.net/stream", "timestamp": 0, "msg_params": {"time": "1h"}}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

Play `/home/puppet/snezhnaya.m4a` till end.

```bash
echo '{"header": "!Puppet93", "token": "MeowMeowMeow", "device_id": "0", "msg_type": "AUD", "msg_data": "file:///home/puppet/snezhnaya.m4a", "timestamp": 0}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

## VID

Plays a video on screen. Video can be hosted online or stored locally on the puppet running host.

Once the video ends, the screen is set to blank (`VOD`).

### Data - msg_data

URL that points to local or remote file

```txt
file:///opt/puppet/hello.mp4
https://samplelib.com/mp4/sample-5s.mp4
```

### Keyboard Controls

`VID` player can be controlled with keyboard controls.

- Seek Forward (+5): Double press right arrow
- Seek Backward (+5): Double press left arrow
- Pause: Press Spacebar
- Repeat: Press `R` or `r`

### Example(s)

Play [https://samplelib.com/mp4/sample-5s.mp4](https://samplelib.com/mp4/sample-5s.mp4) till end

```bash
echo '{"header": "!Puppet93", "token": "MeowMeowMeow", "device_id": "0", "msg_type": "VID", "msg_data": "https://samplelib.com/mp4/sample-5s.mp4", "timestamp": 0}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

Play `/home/puppet/kurzgesagt.mp4` till end.

```bash
echo '{"header": "!Puppet93", "token": "MeowMeowMeow", "device_id": "0", "msg_type": "VID", "msg_data": "file:///home/puppet/kurzgesagt.mp4", "timestamp": 0}' | socat - UDP-DATAGRAM:10.0.255.255:8888,broadcast
```

## About 🐈‍⬛

```txt
puppet
Last Updated: 22 Sep 2026
```
