# Rummage

Rummage is a Card Game project that uses the Bevy game engine. This project includes various modules such as `card`, `mana`, and `player`.

## Building the Project

To build this project, follow these steps:

1. **Install Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

2. **Clone the Repository**: Clone the project repository to your local machine.
    ```sh
    git clone git@github.com:tyler274/rummage.git
    cd rummage
    ```

3. **Install Dependencies**: Run the following command to install the necessary dependencies.
    ```sh
    cargo build
    ```

4. **Build the Project**: Use the following command to build the project in release mode.
    ```sh
    cargo build --release
    ```

5. **Run the Project**: After building, you can run the project using:
    ```sh
    cargo run
    ```

## Project Structure

- `src/`
  - `card.rs`: Contains the implementation of the card module.
  - `mana.rs`: Contains the implementation of the mana module.
  - `player.rs`: Contains the implementation of the player module.
  - `main.rs`: The entry point of the application.

## License

MIT License