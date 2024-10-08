# User Documentation for the Advanced Calculator

## NOTE:
I am a student currently working on this project, so the code is not yet fully optimized and may contain bugs.
This application is still majorly under development, so please report any bugs you may discover.

## Introduction

Welcome to the **SHARK Calculator**, a Rust-based advanced calculator that goes beyond simple arithmetic. It supports complex numbers, trigonometric functions, equation solving, and a powerful aliasing system. With features such as automatic bracket completion and step-by-step solution display, this tool is designed to streamline both basic and advanced calculations.

## Key Features

### 1. **Autocomplete Brackets**
   - Automatically completes open brackets to prevent mismatched parentheses and ensure expressions are well-formed.
   - This feature simplifies handling complex expressions by reducing user errors.

### 2. **Trigonometric Functions**
   - The calculator supports common trigonometric functions like `sin`, `cos`, `tan`, and their hyperbolic counterparts (`sinh`, `cosh`, `tanh`).
   - You can toggle between radians and degrees mode for trigonometric calculations via the "Radian Mode" in the **Modes Menu**.

### 3. **Step-by-Step Solutions**
   - The calculator provides detailed, step-by-step simplifications for equations. This allows users to follow the transformation of expressions or equations during the calculation process.

### 4. **Equation Solving**
   - SHARK Calculator can solve basic and polynomial equations.
   - The calculator automatically identifies the side of the equation with the variable and works on isolating it step by step.
   - It supports both linear and polynomial equation solving (up to a certain degree).

### 5. **Alias System (Huffman-like Encoding)**
   - You can define aliases for numbers or operators using a Huffman-like coding scheme.
   - Aliases allow you to substitute frequently used numbers or symbols with shorter, custom representations to speed up input.
   - Example: Assigning `6` to alias `s`, `7` to alias `se`, and so on as defined in the code.

   #### How to Define Aliases:
   - The default aliases are already set in the app (e.g., `6 = s`, `7 = se`, `+ = p`, etc.), but you can update these as needed.
   - To view the current aliases, type `aliases` in the input prompt.

### 6. **Modes Menu**
   - The calculator comes with different operation modes:
     - **Radian Mode**: For trigonometric calculations in radians.
     - **Alias Input**: Enables the use of aliases in input.
   - To toggle modes, type `modes` in the input prompt and select the respective mode by its number.

## How to Use

### Basic Input
- Simply type your expression or equation and hit Enter.
- Example: Typing `2 + 3` will output the result as `5`.

### Step-by-Step Equation Solution
- Type your equation (e.g., `x + 3 = 5`) and SHARK will provide the steps for isolating `x`.
- The app can handle polynomial equations, simplifying and solving step-by-step.

### Using Aliases
- Activate aliasing mode by entering the **Modes Menu** with `modes` and enabling `Alias Input`.
- Once active, you can replace numbers or operators with their defined aliases.
- Example: If alias `6 = s`, typing `s + 3` will be treated as `6 + 3`.

### Commands
- **`help`**: Displays a help page with information on how to use the calculator's features.
- **`modes`**: Accesses the **Modes Menu** to toggle Radian, Alias, and Equation Solver modes.
- **`aliases`**: Displays all the currently active aliases.
- **`q`**: Exits the calculator.

### Input History
- SHARK Calculator stores your previous inputs in a history file (`.history.txt`), allowing you to recall past calculations.
- Use the up and down arrow keys to browse through your input history.

## Error Handling
- If the calculator encounters invalid input, it will return an appropriate error message and continue running.
- If the application is interrupted (e.g., by pressing `Ctrl+C`), the session will exit gracefully.

---

With these features, SHARK Calculator enhances the user experience with intelligent input handling, powerful math operations, and flexible customization through aliases and modes. Enjoy your calculations!
