# png-rs
Exploring PNG file format

```png-cli image.png -h```

Print help

```png-cli image.png -i```

Prints info about PNG chunks

```png-cli image.png -m secret -e end```

Encodes message: "secret" at the end of PNG file

```png-cli image.png -m secret -e chunk```

Encodes message: "secret" at the custom chunk (sMSG)

```png-cli image.png -m secret -e end -o secret```

Specifies name for output file

```png-cli image.png -d```

Tries to decode message
