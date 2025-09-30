# Razen User Authentication System

A comprehensive user authentication system built in Razen that demonstrates various language features including structs, modules, loops, arrays, and more.

## Features Demonstrated

### Language Features
- **Structs**: Custom `User` and `AuthSystem` data types
- **Module System**: Organized code across multiple modules with imports/exports
- **Loops**: `for` loops for iteration, `while` loops for main program flow
- **Arrays**: Dynamic user storage and manipulation
- **Pattern Matching**: `match` statements for menu handling
- **String Interpolation**: `f"..."` syntax for formatted output
- **Type Conversion**: `.toint()`, `.len()` methods
- **Function Composition**: Modular function design

### Authentication Features
- **User Registration**: Username, email, and password validation
- **User Login**: Secure authentication with attempt limiting
- **Password Security**: Simple password hashing simulation
- **Account Locking**: Automatic locking after failed attempts
- **Session Management**: Login/logout functionality
- **User Profile**: View and edit user information
- **Admin Features**: View all users and system statistics

## Project Structure

```
Razen_Auth_Example/
├── main.rzn          # Main application with UI and flow control
├── user.rzn          # User struct and user-related functions
├── auth_system.rzn   # Authentication logic and user database
├── utils.rzn         # Utility functions for UI and input handling
└── README.md         # This documentation
```

## Module Overview

### `user.rzn`
- Defines the `User` struct with username, email, password hash, status, and login attempts
- Provides user creation, validation, and utility functions
- Implements email validation and password security functions

### `auth_system.rzn`
- Defines the `AuthSystem` struct that manages the user database
- Handles user registration, login, logout, and account management
- Implements security features like attempt limiting and account locking
- Provides user search and statistics functions

### `utils.rzn`
- UI utility functions for formatting and user interaction
- Input handling and validation helpers
- Screen management and user experience functions

### `main.rzn`
- Main application entry point and flow control
- Menu systems for both authenticated and unauthenticated users
- Integration of all modules to create a complete application

## How to Run

```bash
cd examples/Razen_Auth_Example
razen run main.rzn
```

## Usage Example

1. **Register a new user**:
   - Choose option 1 from main menu
   - Enter username (minimum 3 characters)
   - Enter valid email address
   - Enter password (minimum 6 characters)
   - Confirm password

2. **Login**:
   - Choose option 2 from main menu
   - Enter your username and password
   - Access the user dashboard

3. **User Dashboard**:
   - View your profile information
   - Change your password
   - View system statistics
   - Logout when done

## Security Features

- **Password Validation**: Minimum length requirements
- **Email Validation**: Basic format checking
- **Account Locking**: Automatic locking after 3 failed login attempts
- **Password Hashing**: Simulated secure password storage
- **Session Management**: Proper login/logout flow

## Technical Highlights

This example showcases:
- **Memory-only data storage** (no file I/O required)
- **Modular architecture** with clean separation of concerns
- **Error handling** with user-friendly messages
- **Input validation** at multiple levels
- **State management** for authentication sessions
- **Professional UI** with formatted menus and feedback

## Educational Value

This project demonstrates how to build a complete application in Razen using:
- Object-oriented design with structs
- Functional programming with pure functions
- Modular programming with imports/exports
- Control flow with loops and conditionals
- Data structures with arrays and custom types
- User interface design with formatted output
- Security concepts in authentication systems

Perfect for learning Razen language features while building a practical application!


lsiten razen does not has the len() function so we need to add some functions ok. to suport this.

and isten razen has not too much features but this si actully good so we can see and fin the mistakes and fxi them and amke the razen this level.