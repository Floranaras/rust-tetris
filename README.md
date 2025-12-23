# Rust Tetris

A classic Tetris implementation in Rust featuring Game Boy-inspired aesthetics 

## Overview

This project demonstrates clean software architecture principles in game development, with complete separation of game logic, rendering, and input handling. The codebase is structured for maintainability, testability, and cross-platform deployment.

## Features

- Classic Tetris gameplay mechanics
- Seven-bag randomizer for fair piece distribution
- Ghost piece preview system
- Progressive difficulty scaling
- Game Boy color palette and visual styling
- Comprehensive unit test coverage

## Technical Architecture

### Project Structure

```
src/
├── main.rs              # Application entry point and game loop
├── lib.rs               # Public library interface
├── config.rs            # Configuration constants
├── input.rs             # Input handling abstraction
├── renderer.rs          # Rendering system
├── game/
│   ├── mod.rs          # Game module interface
│   ├── board.rs        # Board state and collision detection
│   ├── state.rs        # Game state management
│   └── scoring.rs      # Scoring and level progression
└── tetromino/
    ├── mod.rs          # Tetromino module interface
    ├── types.rs        # Piece type definitions
    ├── piece.rs        # Piece behavior and transformations
    └── bag.rs          # Seven-bag randomization system
```

### Design Principles

- **Separation of Concerns**: Game logic is independent of rendering and input systems
- **Modularity**: Each component has a single, well-defined responsibility
- **Testability**: Core game logic can be tested without graphics initialization
- **Extensibility**: New features can be added with minimal impact to existing code

## Building and Running

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Desktop Build

```bash
# Development build
cargo run

# Optimized release build
cargo build --release
./target/release/rust-tetris
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test game::board
```

## Controls

| Input | Action |
|-------|--------|
| A, Left Arrow | Move piece left |
| D, Right Arrow | Move piece right |
| W, Up Arrow | Rotate piece clockwise |
| S, Down Arrow | Soft drop (accelerated fall) |
| Space | Hard drop (instant placement) |
| H | Toggle help display |
| Space (Game Over) | Restart game |

## Game Mechanics

### Scoring System

| Lines Cleared | Base Points | Multiplier |
|---------------|-------------|------------|
| 1 (Single) | 40 | Current Level |
| 2 (Double) | 100 | Current Level |
| 3 (Triple) | 300 | Current Level |
| 4 (Tetris) | 1200 | Current Level |

### Level Progression

- Level increases every 10 lines cleared
- Drop speed accelerates with each level
- Minimum drop speed threshold prevents excessive difficulty

### Seven-Bag Randomizer

Implements the modern Tetris randomization algorithm:
- All seven piece types appear once per bag
- Ensures fair distribution and reduces extended droughts
- New bag is shuffled when exhausted

## Configuration

All game constants are centralized in `src/config.rs`:

- Board dimensions (10×20 grid)
- Visual settings (block size, offsets, colors)
- Timing parameters (drop speeds, level scaling)
- Scoring values

## Dependencies

- **macroquad** (0.4): Cross-platform game framework
  - Window management
  - Graphics rendering
  - Input handling
  - Frame timing

## Testing

The project includes comprehensive unit tests covering:

- Board collision detection and validation
- Line clearing mechanics
- Piece movement and rotation
- Scoring calculations and level progression
- Seven-bag randomizer distribution
- Game state transitions

Test coverage: 43 passing tests across all modules.

## Performance

- Optimized release builds target minimal binary size
- Link-time optimization (LTO) enabled
- Zero-cost abstractions throughout
- Efficient rendering with minimal draw calls

## Code Quality

- Zero compiler warnings in release builds
- Consistent code formatting
- Comprehensive inline documentation
- Clear separation between public and private interfaces

## Future Enhancements

Potential additions to the codebase:

- Hold piece mechanism
- Pause functionality
- Persistent high score storage
- Audio system integration
- Multiple visual themes
- Multiplayer support
- Mobile touch controls
- WebAssembly compilation target

## Development

### Project Goals

This implementation prioritizes:

1. Code clarity and maintainability
2. Architectural best practices
3. Comprehensive test coverage
4. Platform independence of core logic
5. Professional development standards

### Contributing

When contributing, please:

- Follow existing code structure and naming conventions
- Add tests for new functionality
- Update documentation as needed
- Ensure `cargo test` passes
- Verify `cargo clippy` produces no warnings

## License

MIT License - See LICENSE file for details

## Acknowledgments

Built with Rust and the Macroquad game framework. Inspired by classic Game Boy Tetris aesthetics and modern software architecture principles.

---
