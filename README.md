# Dear

Dear is a **yet another note taking app**.

It was born mainly from my desire for learning Rust... I realised that working on a personal project that I could actually use and improve while it became more complex would be a great fun learning experience.

## Installation

**Obs.: Still there is no published binary**

Using cargo
```bash
cargo install dear
```

## Usage

Dear is currently able to **save** and **list** Notes. They are stored as plain text in a configurable location.
A Note has a title and it might have a description and any number of tags.

### Configuration

TODO

### Saving a Note

```bash
# Minimal Note
dear save --title "My First Note"

# Note with description
dear save --title "My Second Note" -d "I'm just testing out this, dear"

# Notes might have several tags
dear save --title "Rust is amazing" -t rust
dear save --title "My dog's schedule" -d "Walk at 9 and 18 | Eat at 12 and 20" -t dog -t home
```

### List Notes

```bash
dear list
[Note { title: "My dog's schedule", description: Some("Walk at 9 and 18 | Eat at 12 and 20"), tags: Some(["dog", "home"]) }, Note { title: "My First note", description: None, tags: None }, Note { title: "Rust is amazing", description: None, tags: Some(["rust"]) }, Note { title: "My Second Note", description: Some("I'm just testing out this, dear"), tags: None }]
```

## Contribution Guidelines

TODO
