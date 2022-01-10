# png-rs
Exploring PNG file format

# Usage

Print help

```png-cli image.png -h```

Prints info about PNG chunks

```png-cli image.png -i```

Encodes message: "secret" at the end of PNG file

```png-cli image.png -m secret -e end```

Encodes message: "secret" at the custom chunk (sMSG)

```png-cli image.png -m secret -e chunk```

Specifies name for output file

```png-cli image.png -m secret -e end -o secret```

Tries to decode message

```png-cli image.png -d```
