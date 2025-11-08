# Murmure - Technical Guidelines

## Introduction

The goal of this document is to compile the development practices for the Murmure repository, aiming for more consistent, secure, and maintainable development.

During code reviews, developers must ensure adherence to these established rules. Where possible, these rules will also be enforced by linters (`ESLint` for the frontend, `Clippy` for the backend).

---

## Frontend Guidelines (React + TypeScript)

### 1. Naming Conventions

#### 1.1 Files and Folders

File and folder names must be in **kebab-case**.
-   **Components**: `my-component.tsx`
-   **Hooks**: `use-audio-recorder.ts`
-   **Helpers**: `my-component.helpers.ts`

> **Rationale**: Enhances readability and avoids case-sensitivity issues across different operating systems (Windows, macOS, Linux).

#### 1.2 Component and Hook Naming
-   **Components** must be named in **PascalCase**.
    -   Example: `UpdateChecker`, `Settings`.
-   **Custom Hooks** must be prefixed with `use` and written in **camelCase**.
    -   Example: `useWakeup`, `useGlobalShortcut`.
-   **Helpers** stored a list of public functions in **camelCase**.
    -   Example: `formatTime` in `history.helpers.ts`.

> **Rationale**: These are standard conventions within the React ecosystem, enforced by linters to correctly identify hooks and components.

#### 1.3 Interfaces and Types

-   **Interfaces** and **type aliases** must be in **PascalCase**.
-   Do not prefix interfaces with `I`.
-   Always prefer interfaces over type aliases when you can.
-   Use interface for defining the shape of objects and for component props. 
-   Use type for defining union types, tuples, or complex types derived from others.

    ```typescript
    // Recommended
    interface TranscriptionResult {
      text: string;
      timestamp: number;
    }

    // Avoid
    type ITranscriptionResult = {
      // ...
    }
    ```

> **Rationale**: Follows TypeScript community standards and improves code clarity.

### 2. Folder and File Structure

#### 2.1 Feature-First Project Structure

Organize the code by feature to improve modularity and maintainability.
-   **`/src/components/`**: For shared, reusable atomic UI components and shadcn components (e.g., `Button`, `Card`, `Tooltip`).
-   **`/src/components/hooks/`**: For globally shared custom hooks or for atomic/shadcn components.
-   **`/src/components/lib/`**: For global services, utilities, and external library configurations.
-   **`/src/features/{page/feature}`**: For distinct application features or pages. Each feature folder contains all related components, hooks, and logic.
-   **`/src/features/{page}/{page}.tsx`**: Feature/Page Entry Point. (e.g., `history.tsx`).
-   **`/src/features/{page}/{page}.helpers.ts`**: Specific helpers functions for the page. (e.g., `history.helpers.ts`).
-   **`/src/features/{page}/hooks/use-{hook}.ts`**: Specific hooks for the page. (e.g., `use-history-state.ts`).
-   **`/src/features/{page}/{specific-component}/{specific-component}.tsx`**: Specific components for the page (e.g., `history.tsx`).
-   **`/src/features/{page}/{specific-component}/hooks/use-{hook}.ts`**: Specific hooks for the specific component (e.g., `use-history-item-state.ts`).

src/
├── components/
│   ├── hooks/
│   │   └── use-mobile.ts      # Règle: /src/components/hooks/
│   ├── lib/
│   │   └── utils.ts                # Règle: /src/components/lib/
│   ├── button.tsx                  # Règle: /src/components/
│   └── card.tsx                    # Règle: /src/components/
│
└── features/
    └── history/                    # Règle: /src/features/{page/feature}
        ├── hooks/
        │   └── use-history-state.ts  # Règle: /src/features/{page}/hooks/use-{hook}.ts
        │
        ├── history-item/
        │   └── history-item.tsx      # Règle: /src/features/{page}/{specific-component}/{specific-component}.tsx
        │
        ├── history.helpers.ts      # Règle: /src/features/{page}/{page}.helpers.ts
        └── history.tsx             # Règle: /src/features/{page}/{page}.tsx (Point d'entrée)

#### 2.2 Avoid Barrel Files (`index.ts`)

Do not use barrel files to re-export modules from a directory. Import directly from the source file.

> **Rationale**: Barrel files can negatively impact tree-shaking and lead to slower build times and larger bundle sizes. Direct imports ensure that only the necessary code is included.

### 3. React & Component Best Practices

#### 3.1 Component Definition

Define components as functions. Class components are not allowed.

```tsx
// Avoid using React.FC. Type props directly.

// Recommended
interface GreetingProps {
  name: string;
}

const Greeting = ({ name }: GreetingProps) => {
  return <h1>Hello, {name}!</h1>;
};
```

> **Rationale**: Function components with hooks are the modern standard. Avoiding React.FC provides better type safety and is less verbose.

#### 3.2 Styling (Tailwind CSS + shadcn/ui)

-   **UI Components**: Use shadcn/ui as the base component library.
-   **Styling**: Use Tailwind CSS utility classes for all styling. Avoid plain .css or .scss files for component-specific styles.
-   **Icons**: Use icons from the lucide-react library.

> **Rationale**: This stack ensures a consistent, maintainable, and highly customizable design system with excellent performance.

#### 3.3 State Management

-   **Simple Local State**: Use useState and useReducer.
-   **Complex Global State**: Use Zustand. It offers a simple, powerful, and unopinionated API with less boilerplate than Redux.

####3.4 Accessibility (a11y)

-   **Semantic HTML**: Always prefer semantic HTML elements (<button>, <nav>, <main>) over generic ones (<div>, <span>) to ensure a meaningful structure.
-   **ARIA Attributes**: Use ARIA attributes (aria-label, role, etc.) when native semantics are not sufficient, especially for custom components.

> **Rationale**: Building an accessible application from the start is easier than retrofitting it later and ensures a better experience for all users.

### 4. TypeScript Usage

#### 4.2 Avoid any; Prefer unknown

-   The use of `any` is strictly forbidden. It disables type checking and compromises safety.
-   Use guard type, type guards, or type assertions to ensure the type of the variable is correct.
-   Use `unknown` only as a last resort, when the type of a variable cannot be determined.

```tsx
interface HistoryEntry {
    id: number;
    timestamp: number;
    text: string;
}

export const useHistoryState = () => {
    // ...
    const loadHistory = async () => {
        try {
            const entries = await invoke<HistoryEntry[]>(
                'get_recent_transcriptions'
            );
            setHistory(entries);
        } catch (e) {
            console.error('Failed to load history:', e);
        }
    };
    // ...
}
```

#### 4.3 Explicit Conditions
Avoid implicit truthiness checks. Be explicit about the condition you are checking.
```tsx
// Recommended
const items: string[] = [];
if (items != null) {
    if (items.length > 0) {
    // ...
    }
}

// Avoid
if (items) { // Implicitly checks if items is not null
    if (items.length) { // Implicitly checks if length is not 0
    // ...
    }
}
```

## Backend Guidelines (Rust + Tauri)

### 1. Naming Conventions

-   **Follow the official Rust API Guidelines.**
-   **Modules, crates, functions, variables**: snake_case (e.g., transcription_engine, fn start_recording() {}).
-   **Types (Structs, Enums, Traits)**: PascalCase (e.g., struct AppState, enum AppError {}).
-   **Constants**: UPPER_SNAKE_CASE (e.g., const MAX_HISTORY: usize = 5;).

> **Rationale**: These are idiomatic Rust conventions enforced by the compiler and Clippy. Adhering to them makes the code readable for any Rust developer.

### 2. Code Organization & Structure
#### 2.1 Modular Architecture

Structure the backend code in src-tauri/src/ into logical modules.
-   **`/src-tauri/src/`**: The main backend directory.
-   **`/src-tauri/src/commands.rs`**: The main commands module.
-   **`/src-tauri/src/state.rs`**: The main state module.
-   **`/src-tauri/src/error.rs`**: The main error module.
-   **`/src-tauri/src/model.rs`**: The main model module.
-   **`/src-tauri/src/audio.rs`**: The main audio module.

main.rs: The application entry point. Keep it minimal. It should primarily be used for building and running the Tauri application.
commands.rs: A dedicated module for all Tauri #[tauri::command] functions exposed to the frontend.
state.rs: Defines the shared application state (tauri::State).
error.rs: Defines custom error types for the application.
Other logical modules as needed (e.g., audio.rs, model.rs).

src-tauri/src/
├── main.rs
├── commands.rs
├── state.rs
├── error.rs
├── model.rs
└── audio.rs

> **Rationale**: Separating concerns into modules makes the codebase easier to navigate, maintain, and test.

### 3. Error Handling
#### 3.1 Use Result<T, E> for All Fallible Operations

-   Never use panic! for recoverable errors. panic! should only be used for unrecoverable states that indicate a bug in the program.
-   Functions that can fail must return a Result<T, E>.

```rs
pub fn set_record_shortcut(app: AppHandle, binding: String) -> Result<String, String> {
    let keys = parse_binding_keys(&binding);
    if keys.is_empty() {
        return Err("Invalid shortcut".to_string());
    }
    // ...
    Ok(normalized)
}
```

### 4. Tooling and Code Quality

#### 4.1 Clippy and rustfmt

-   **Clippy**: The Rust linter is mandatory. Always run cargo clippy and fix all warnings before committing code.
-   **rustfmt**: All code must be formatted with cargo fmt. This is usually handled automatically by the IDE.

> **Rationale**: These tools enforce idiomatic Rust and a consistent code style across the entire project, significantly improving code quality and readability.

#### 4.2 Dependency Management

-   Be mindful of adding new dependencies. Each dependency increases compile time and binary size.
-   Regularly run cargo audit to check for security vulnerabilities in dependencies. This is crucial for a privacy-focused application like Murmure.

> **Rationale**: These tools ensure the security and maintainability of the project.

### 5. Control Flow

#### 5.1 Prefer `match` for Pattern Matching

For complex conditional logic, especially when dealing with enums or different states of a value, prefer using `match` expressions over `if let` / `else if` chains.

The Rust compiler enforces that `match` statements are **exhaustive**, meaning all possible cases must be handled. This is a powerful safety feature that prevents bugs from unhandled states.

```rust
// Consider this enum
enum AppEvent {
    StartRecording,
    StopRecording,
    SetModel(String),
}

let event = AppEvent::StartRecording;

// Avoid complex if/else if chains
if let AppEvent::StartRecording = event {
    println!("Starting recording...");
} else if let AppEvent::StopRecording = event {
    println!("Stopping recording...");
} // What if a new event is added? This chain might not be updated.


// Recommended: Use match for clarity and compile-time exhaustiveness checks
match event {
    AppEvent::StartRecording => {
        println!("Starting recording...");
    }
    AppEvent::StopRecording => {
        println!("Stopping recording...");
    }
    AppEvent::SetModel(model_name) => {
        println!("Setting model to: {}", model_name);
    }
    // The compiler will error if a new variant is added to AppEvent
    // and not handled here.
}
```

> **Rationale**: `match` is more idiomatic and expressive for pattern matching in Rust. Its main advantage is the compile-time guarantee of exhaustiveness, which eliminates a common source of bugs and makes code safer and easier to refactor.