# Puppet 🎭

A simple program to receive instructions from a server via UDP broadcasts, then decode them to do the actions as instructed.

## Instructions

Instructions are JSON data trasmitted as UTF-8 string via UDP broadcasts. Puppet expects the JSON to follow a strict format.

### Fields

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

- header: Denotes a group. A base level check made by puppet, header has to match the value by the same name configured in puppet
- token: A simple string to block intruders (TODO: Token is not hashed as of writing this)
- device_id: ID that identifies the puppet host. Is a string
- msg_type: Classifies instruction types
  - TXT - Text
  - AUD - Audio
  - VID - Video (Coming Soon...)
  - YTA - YouTube Audio (Coming Soon...)
  - YTV - YouTube Video (Coming Soon...)
- msg_data: Data for the action to process (For YTA, it is the YouTube URL)
- msg_params: Extra params for the actions
- timestamp: UNIX EPOCH for puppet to instruct when to start the action (good for precision)

### Sample Command

The following command sends a sample UDP broadcast on linux using the command `socat`. This can be picked by puppet.

```bash
echo '{"header": "!Puppet93", "token": "MeowMeowMeow", "device_id": "0", "msg_type": "AUD", "msg_data": "https://yumicoradio.net/stream", "msg_params": ["30s"], "timestamp": 1785863627}' | socat - UDP-DATAGRAM:10.229.99.255:8888,broadcast
```

This command is to instruct puppet to stream [yumicoradio.net](https://yumicoradio.net) at timestamp 1785863627 for 30 seconds.

## Server

There is no recognised dedicated server program for this purpose as of writing this. You can however write a server in any language to send JSON as UDP broadcasts matching the schema explained above. 

## Some Questions You May Have

**Q: What is this?**

**A:** You scrolled too far. Scroll up and read from the start

**Q: Why did you make this?**

**A:** Each and every event in this world happens for a reason. Naturally, I too had to make this to coordinate lab systems for an event hosted at the insitution where I am doing my under graduation from. I saw the need for a solution to help event coordinates control lab systems as per their needs - this project as of now is mostly aimed at presentation ie. puppet as of writing this is good for displaying images, playing audio, video etc.

**Q: How can I use this?**

**A:** As explained above, you can use this software along with a UDP transmistter program to control many desktops at once, coordinate them to perform certain actions. Naturally, all actions you can think of are not included in the program. Feel free to contribute morally acceptable ones to this project.

**Q: Can't I misuse this?**

**A:** Unfortunately for me, yes. Most things you get from this world can be misused. I do not condone misuse and you must not perform, condone or promote it either. However by design, this software does not the lock the client. The client can always stop this program without much hassle.

**Q: Can I use this program to force control the clients?**

**A:** No, this software is not for restricting the client. There are other programs to do that temporarily to conduct examinations and protect organisation assets.

**Q: Why is this written in Rust?**

**A:** Excluding the many advantages of Rust including but not limited to cross-compatability, memory safety, excellent library management etc, I do like to code in Rust.

## About

```txt
puppet pre-alpha
Last Updated: 04 Aug 2026

ペンギン
```
