# Documentation Guidelines

This page contains the comprehensive documentation guidelines for the Rummage MTG Commander game engine built with Bevy 0.15.x.

## Documentation Structure

The documentation is organized into the following major sections:

1. **[Commander Rules](../formats/commander/index.md)** - Implementation of MTG Commander format rules and mechanics
2. **[Game UI](../game_gui/index.md)** - User interface systems and components
3. **[Networking](../networking/index.md)** - Multiplayer functionality using bevy_replicon

## Contributing to Documentation

When contributing to the documentation, please follow these guidelines:

### Document Structure

Each document should have:

1. A clear, descriptive title
2. A brief introduction explaining the document's purpose
3. A table of contents for documents longer than a few paragraphs
4. Properly nested headings (H1 -> H2 -> H3)
5. Code examples where appropriate
6. Cross-references to related documents
7. Implementation status indicators where appropriate

### Style Guide

1. Use American English spelling and grammar
2. Write in a clear, concise style
3. Use active voice where possible
4. Keep paragraphs focused on a single topic
5. Use proper Markdown formatting:
   - `#` for headings (not underlines)
   - Backticks for code snippets
   - Triple backticks for code blocks with language identifier
   - Dashes for unordered lists
   - Numbers for ordered lists

### Code Examples

Include meaningful code examples that:

1. Use non-deprecated Bevy 0.15.x APIs
2. Demonstrate the concept being explained
3. Are syntactically correct
4. Include comments for complex code
5. Use Rust syntax highlighting with ````rust`

### Implementation Status

Use the following indicators to mark implementation status:

- ✅ Implemented and tested
- 🔄 In progress
- ⚠️ Planned but not yet implemented

## Building the Documentation

We use `mdbook` to build the documentation. To get started:

1. Install required tools:
   ```bash
   make install-tools
   ```

2. Build the documentation:
   ```bash
   make build
   ```

3. View the documentation locally:
   ```bash
   make serve
   ```

Then open your browser to http://localhost:3000

## Documentation Maintenance

Use the following make commands for documentation maintenance:

- `make lint` - Check for style issues
- `make toc` - Generate table of contents
- `make check` - Check for broken links
- `make validate` - Validate documentation structure
- `make update-dates` - Update last modified dates

## Questions?

If you have questions about the documentation, please contact the documentation team or open an issue in the project repository. 