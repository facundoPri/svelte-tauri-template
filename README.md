# Svelte + Tauri Floating Panel Template

A template/example to show how to create an app that uses floating panels using Tauri 2 and Svelte on MacOs.
The app have 2 main windows:
1. A regular main window that appears in the dock
2. A floating panel window that can be toggled with keyboard shortcuts and is visible in all spaces including fullscreen spaces.

## Features

- **Floating Panel**: A macOS-style floating panel that can be toggled with keyboard shortcuts (Alt+Space)
- **Fullscreen Compatible**: Panel remains visible even in fullscreen spaces
- **Follows you around the spaces**: The panel will follow you around the spaces and will be visible in all of them.
- **Regular Window**: A regular window that appears in the dock and can be used for the main application UI

## Tech Stack

- [Tauri 2](https://tauri.app/) - Desktop application framework
- [Svelte](https://svelte.dev/) & [SvelteKit](https://kit.svelte.dev/) - Frontend framework
- [nspanel](https://github.com/ahkohd/tauri-nspanel) - Tauri plugin for macOS panel functionality
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- [shadcn/ui](https://ui.shadcn.com/) - UI component library

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/) (latest stable)
- [pnpm](https://pnpm.io/) (recommended) or npm/yarn
- [Tauri CLI](https://tauri.app/) (v2)

### Installation

1. Clone this repository:

```
git clone https://github.com/facundoPri/svelte-tauri-template.git
```

2. Navigate to the project directory:

```
cd svelte-tauri-template
```

3. Install dependencies and run the demo:

```
pnpm install
pnpm tauri dev
```

4. Press Alt+Space to toggle the spotlight window

## Usage

1. Launch the application
2. Press `Alt+Space` to toggle the floating panel
3. The panel will appear and follow you around the spaces
4. Type something and press Enter or click the "Greet" button

## Project Structure

- `src/` - Svelte frontend code
  - `routes/` - SvelteKit routes
    - `panel/` - Panel UI components
  - `lib/` - Shared components and utilities
- `src-tauri/` - Rust backend code
  - `src/` - Rust source files
    - `lib.rs` - Main application logic
    - `panel.rs` - Panel functionality
    - `command.rs` - Tauri commands


### Styling the Panel

The panel UI is defined in `src/routes/panel/+page.svelte`. You can modify this file to change the appearance and behavior of the panel.

## Adapting for Other Frameworks

While this example uses Svelte, you can adapt the Tauri code to work with any frontend framework supported by Tauri 2.

Simply copy the Tauri configuration and Rust code to your project and adjust the frontend accordingly.

## Special Thanks

This project wouldn't be possible without the contributions of:

- The [Tauri Team](https://github.com/tauri-apps) for creating an amazing framework
- [ahkohd](https://github.com/ahkohd) for creating the [tauri-nspanel](tauri-nspanel) plugin and [tauri-macos-spotlight-example](tauri-macos-spotlight-example)
- [zzzze](https://github.com/zzzze) for creating the [tauri-plugin-spotlight](https://github.com/zzzze/tauri-plugin-spotlight)

## License

MIT
