
# Welcome to the PopperASM project! This readme provides an overview of the project, its components, and how to get started.

# Table of Contents
- [Introduction](#introduction)
- [Project Goals](#project-goals)
- [Components](#components)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
# Introduction
PopperASM is a Rust-based project that implements an assembler for the Popper-lang programming language. This assembler processes PopperASM source code, performing lexical analysis, parsing, and generating an Abstract Syntax Tree (AST). The AST can then be used to generate machine code suitable for execution on the PopperCPU architecture.

# Project Goals
The primary objectives of the PopperASM project are:

1. **Assembler Implementation**: Create a robust and efficient assembler in Rust that can process PopperASM source code.

2. **Lexer and Parser**: Develop a lexer to tokenize the input code and a parser to generate an Abstract Syntax Tree (AST) representing the code's structure.

3. **AST Generation**: Construct an AST that captures the hierarchical structure of PopperASM code for further processing.

4. **Machine Code Generation**: Utilize the AST to generate machine code that is compatible with the instruction set of the PopperCPU.

5. **Documentation**: Provide clear and comprehensive documentation to guide users on writing PopperASM code and using the assembler.

# Components
The PopperASM project comprises the following components:

- **Lexer**: The lexer analyzes the source code and tokenizes it into meaningful units, such as keywords, operands, and symbols.

- **Parser**: The parser processes the tokens produced by the lexer and constructs an Abstract Syntax Tree (AST) that represents the syntactic structure of the code.

- **Abstract Syntax Tree (AST)**: The AST is a hierarchical representation of the code's structure, capturing the relationships between different elements.

- **Machine Code Generator**: Using the information in the AST, the machine code generator produces binary code that can be executed on the PopperCPU.

# Getting Started
To start using the PopperASM project:

1. Clone the project repository: `git clone https://github.com/popper-lang/PopperASM.git`

2. Ensure you have Rust and its dependencies installed on your system.

3. Review the documentation in the docs directory to understand how the lexer, parser, and AST work together.

# Usage
1. Write your PopperASM code in a `.popasm` file using your preferred text editor.

2. Open a terminal and navigate to the directory containing the Rust source files.

3. Build and run the PopperASM assembler on your code: `cargo run input.popasm` 

The assembler processes the input code, generates the AST, and printed it into json

# Contributing
Contributions to the PopperASM project are highly encouraged! To contribute:

1. Fork the repository and create a new branch.

2. Implement improvements to the lexer, parser, AST, or machine code generator.

3. Submit a pull request, providing a clear description of your changes and their benefits.

# License
This project is licensed under the MIT License.
