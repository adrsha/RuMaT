# SHARK Calculator Documentation

## NOTE:
I am a student currently working on this project, so the code is not yet fully optimized and may contain bugs.
This application is still majorly under development, so please report any bugs you may discover.

## Overview

The **SHARK Calculator** is a Rust-based calculator that supports complex numbers, trigonometric functions, equation solving, and an aliasing system for faster input. It automatically completes brackets and shows step-by-step calculations.

## Features

### 1. **Bracket Autocompletion**
   - Automatically completes open brackets to ensure proper expression formatting.

### 2. **Trigonometric Functions**
   - Supports `sin`, `cos`, `tan`, and their hyperbolic versions (`sinh`, `cosh`, `tanh`).
   - Toggle between radians and degrees in the **Modes Menu**.

### 3. **Step-by-Step Solutions**
   - Displays detailed steps for solving equations or simplifying expressions.

### 4. **Equation Solving**
   - Solves basic and polynomial equations.
   - Handles linear and polynomial equations by isolating variables.

### 5. **Alias System**
   - Assign custom aliases to numbers and operators using a Huffman-like encoding system.
   - Example: `6` can be assigned to alias `s`, `+` to alias `p`.

#### Define Aliases:
   - Default aliases are predefined (e.g., `6 = s`, `+ = p`).
   - Use the command `aliases` to view or manage aliases.

### 6. **Modes Menu**
   - **Radian Mode**: For trigonometric calculations in radians.
   - **Alias Input**: Enables the use of aliases in expressions.
   - **Equation Solver**: Activates the equation-solving feature.
   - Access modes by typing `modes`.

## Usage

### Basic Input
- Enter an expression (e.g., `2 + 3`) and press Enter to calculate.

### Step-by-Step Equation Solving
- Type an equation (e.g., `x + 3 = 5`), and the calculator will show the steps to solve it.

### Using Aliases
- Enable aliasing mode via `modes` and use defined aliases in your input.
- Example: If `6 = s`, then `s + 3` will be evaluated as `6 + 3`.

### Commands
- **`help`**: Displays usage information.
- **`modes`**: Access the **Modes Menu**.
- **`aliases`**: Shows active aliases.
- **`q`**: Quits the calculator.

### Input History
- Use the arrow keys to browse your input history.

## Error Handling
- The calculator will return error messages for invalid input. 
- Exit the app gracefully with `Ctrl+C`.

--- 

This documentation provides the basic functionality and usage instructions for the SHARK Calculator.
