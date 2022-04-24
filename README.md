# Dear

Dear is a **yet another note taking app**.

It was born mainly from my desire for learning Rust... I realised that working on a personal project that I could actually use and improve while it became more complex would be a great fun learning experience.

## Installation

Using cargo
```bash
cargo install dear
```

## Usage

Dear is currently able to **save** and **list** Notes. They are stored as plain text in a configurable location.
A Note has a title and it might have a description and any number of tags.

[![Demo](https://asciinema.org/a/489767.svg)](https://asciinema.org/a/489767)

### Configuration

TODO

### Saving a Note

```bash
# Minimal Note
> dear save --title "My First Note"
Note { title: "My First Note", description: None, tags: None }

# Note with description
> dear save --title "A Note with Description" -d "Description o/"
Note { title: "A Note with Description", description: Some("Description o/"), tags: None }

# Notes might have several tags
> dear save --title "A Note with a Tag" -t tag
Note { title: "A Note with a Tag", description: None, tags: Some(["tag"]) }

> dear save --title "A Note with multiple tags" -t tag1 -t tag2
Note { title: "A Note with multiple tags", description: None, tags: Some(["tag1", "tag2"]) }
```

### List Notes

```bash
> dear list
┌──────────────────────────────────────────────────────────────────────────────┐
│                           A Note with Description                            │
│                                                                              │
│ Description o/                                                               │
│                                                                              │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────────┐
│                              A Note with a Tag                               │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                         [tag]│
└──────────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────────┐
│                                My First Note                                 │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────────┐
│                          A Note with multiple tags                           │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                  [tag1][tag2]│
└──────────────────────────────────────────────────────────────────────────────┘
```

## Contribution Guidelines

TODO
