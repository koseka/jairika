# Contributing to Jairika

[中文](translations/中文/CONTRIBUTING.md) | [Русский](translations/Русский/CONTRIBUTING.md) | [Español](translations/Español/CONTRIBUTING.md) | [Français](translations/Français/CONTRIBUTING.md) | [日本語](translations/日本語/CONTRIBUTING.md) | [한국어](translations/한국어/CONTRIBUTING.md) | [Português](translations/Português/CONTRIBUTING.md) | [Italiano](translations/Italiano/CONTRIBUTING.md) | [हिन्दी](translations/हिन्दी/CONTRIBUTING.md) | [العربية](translations/العربية/CONTRIBUTING.md)

Thank you for your interest in contributing to this project! Below are some guidelines to help maintain consistency and clarity across commits.

## Prerequisites

This project use [Trunk](https://www.trunk.io) to ...

### Code

This repository contains two essential directories:
* [**`library/`**](library/) is the core of the framework (contains the library and the CLI)
* [**`bindings/`**](bindings/) contains all the bindings for the different languages supported by the framework

-> Insert instructions about how to properly contribute to the code of the project and add a link to a detailed guide.

### Documentation

-> Insert some instructions about how to properly contribute to the documentation of the project and add a link to a detailed guide.

## Commit Messages

All commit messages should follow this format:
```
[type] "short description"
```

*	**`[type]`**: The category of the changes you’re making. It should be written in lowercase and enclosed in brackets.
*	**`"short description"`**: A single sentence that summerizes the changes (in english only). The sentence cannot contain more than 50 charaters (alphanumeric and single quotation marks only) and must start with a verb in the past tense. The sentence must start with a capital letter and end with a period.

### Categories and Priority

We use several commit categories, and there is a priority system to ensure that only the most significant change is reflected in the commit message. If a commit involves multiple types of changes, choose the highest-priority category based on the list below.

1. **`[fix]`**: For bug fixes, prevails regardless of the other changes that come with it.
2. **`[feat]`**: For adding new features or functionalities, as long as no bug fixes are involved.
3. **`[perf]`**: For performance improvements, provided no new features or bug fixes are included.
4. **`[chore]`**: For minor changes that don't affect the logic of the code, like updating dependencies, formatting, or improving code structure and readability.
5. **`[test]`**: For adding or updating tests, without impacting the core functionality of the software.
6. **`[docs]`**: For documentation changes, such as updates to the `README.md` files, comments, or API documentation.

### Examples

*	If you fixed a bug and updated the tests, use *fix* as it takes priority e.g. `[fix] Changed x to make y compatible with 'z'.`.
*	If you added a feature and improved code formatting, use *feat* since it ranks higher than style changes e.g. `[feat] Added x to y and removed z.`.

## Additional Guidelines

*	**Atomic Commits**: Each commit should represent a single logical change. If you’re working on multiple types of changes, it’s often better to make separate commits.
*	**Descriptive Messages**: Ensure the short description provides enough information to understand the nature of the change at a glance.

By following these guidelines, we can maintain a clean and structured commit history.
