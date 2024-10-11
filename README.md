![Logo](./assets/banners/project.png)

-> Insert badges from https://shields.io/.

[中文](translations/中文/README.md) | [Русский](translations/Русский/README.md) | [Español](translations/Español/README.md) | [Français](translations/Français/README.md) | [日本語](translations/日本語/README.md) | [한국어](translations/한국어/README.md) | [Português](translations/Português/README.md) | [Italiano](translations/Italiano/README.md) | [हिन्दी](translations/हिन्दी/README.md) | [العربية](translations/العربية/README.md)

## Introduction

**Jairika** is a *blazing fast* cross-platform machine learning framework designed for efficient neural network loading, inference, and training. The framework is composed of a core library and a CLI, as well as several bindings for other languages, all written in [Rust](https://www.rust-lang.org).

The goal of this project is to provide developers with a high-performance machine learning framework that supports multiple programming languages, making it ideal for both backend applications and local devices. Designed with a *"build once, train everywhere"* approach, models can be built and trained in one language like Rust or Python, and then executed or re-trained seamlessly in another language without requiring any adjustments.

A key focus of this initiative is allowing small yet powerful AI models to run on local devices such as phones and computers, or directly within a browser. These models could handle tasks like summarization, translation, speech-to-text conversion, image manipulation, and so much more. By supporting lightweight models in various environments, Jairika unlocks the potential for AI to run efficiently and universally on any platform.

---

**Supported Languages**

* [x] [Rust]() (natively)
* [ ] [Python]() (through [pyo3](https://github.com/PyO3/pyo3))
* [ ] [JavaScript]() (through [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen))
* [ ] [Java]() (through [jni-rs](https://github.com/jni-rs/jni-rs))
* [ ] [Go]() (through [something]())
* [ ] [PHP]() (through [something]())

**Features**

* [ ] Architecture mutability: modify the models architecture at anytime before, during or after training or inference
* [ ] Build and train networks with multiple input layers
* [ ] Build and train networks with multiple output layers
* [ ] Support for building and training CNNs
* [ ] Support for building and training RNNs
* [ ] Model paging: partially load networks into RAM for optimized memory usage
* [ ] Universal GPU acceleration with [Vulkano](https://github.com/vulkano-rs/vulkano?tab=readme-ov-file)
* [ ] Implement [Bend](https://github.com/HigherOrderCO/bend) for massive parallelization during inference and training

## Benchmarks

-> Insert a diagram showing how the framework performs at different tasks in comparison to [TensorFlow](https://www.tensorflow.org) and [PyTorch](https://pytorch.org).

-> Insert a detailed description of the tasks for which the frameworks where compared as well as the material that was used.

## Concepts

Jairika is conceptually inspired by [Git](https://git-scm.com), where models function like repositories with branches and commits. From a technical perspective, this is achieved by using [SurrealDB](https://surrealdb.com/docs/surrealdb) to manage AI model storage and organization.

### Mapping

| **Jairika** | **Git (Conceptually)** | **SurrealDB (Technically)** |
| ----------- | ---------------------- | --------------------------- |
| Model       | Repository             | SurrealKV instance          |
| Address     | Path                   | Address                     |
| Variant     | Branch                 | Namespace                   |
| Iteration   | Commit                 | Database                    |
| Layer       | File                   | Table                       |

### Breakdown

***Model***: Represents the entirety of the AI system being developed or trained. This can be compared to a Git repository, where the model serves as the primary container for all its components (layers, variants, iterations, etc.). From a coding perspective, it's a SurrealDB instance backed by a disk persistent SurrealKV store.

***Address***: The location where the model is stored, analogous to a path in a file system (or a Git remote URL). In practice, the address directs you to a SurrealDB instance.

***Variant***: A version of the model with a specific architecture. This works similarly to how Git handles branches. Programmatically, it's a SurrealDB namespace.

***Iteration***: A neural net with specific parameter values. This is akin to a commit in Git, representing a checkpoint or a snapshot of a variant's parameter values at a given time. At the code level, it's a SurrealDB database.

***Layer***: A set of neurones with their respective connections (this is can be thought off as a file in a Git repo). The biases of the neurones and the weights of their connections are stored in SurrealDB tables.

### Example

```bash
user$ jrk info /Users/user/Desktop/models/chatbot-1B

address: "speedb:///Users/name/Desktop/models/chatbot-1B"
├── variant: "transformer" - created at 2023-10-05T14:23:45Z
│   ├── iteration: "prime" - last trained at 2023-10-05T14:23:45Z
│   ├── iteration: "PT" - last trained at 2023-10-27T00:48:04Z
│   ├── iteration: "FT" - last trained at 2023-11-14T17:59:08Z
│   ├── iteration: "FT-IT" - last trained at 2023-12-21T22:30:51Z
│   └── iteration: "FT-RLHF" - last trained at 2024-04-20T14:23:45Z
├── variant: "lstm" - created at 2023-09-22T19:11:02Z
│   ├── iteration: "prime" - last trained at 2023-09-22T19:11:02Z
│   └── iteration: "PT" - last trained at 2023-10-01T04:35:58Z
└── variant: "mamba" - created at 2023-12-17T20:03:41Z
    ├── iteration: "prime" - last trained at 2023-12-17T20:03:41Z
    ├── iteration: "PT" - last trained at 2024-01-09T12:44:36Z
    └── iteration: "FT" - last trained at 2024-01-31T16:02:15Z
```

## Architecture

Jairika is built using a layered approach, with compute separated from the storage. This allows us to scale the compute and storage layers independently while promoting maintainability and separation of concerns.

## Use Cases

By leveraging SurrealDB for model management and drawing inspiration from Git's control principles, Jairika's design opens up a wide range of use cases and provides several advantages compared to traditional machine learning frameworks, including:

* **Managing Different Architectures**: Easily create and manage multiple versions of the model, each with its own unique architecture and manipulate them independently. This allows you to explore multiple approaches without overwriting the existing versions of the model.

* **Creating Checkpoints and Version Control**: Track the state of model variants over time, experiment with different training methods, revert, or inspect specific steps in the training process, and maintain a clear history of changes.

* **Improved Collaboration**: Facilitate team contributions by organizing model development in a structured and trackable way, enhancing collaborative efforts.

* **Optimized Memory Usage**: Neural networks can be partially loaded into RAM, allowing efficient training and inference even with memory limitations or when working with heavy models.

* **Enhanced Security**: Storing models in a database enables fine-grained control over read/write access, improving the security of the model's parameters and architecture.

* **Cost Effective and Secure Real-Time Deployment**: Deploy models in real-time across multiple platforms, allowing immediate testing and scaling without the need for reconfiguration. And, with the *distributed inference* feature, you can securely off load parts of the model to the client for inference, useful for reducing server load and inference costs.

* **Universality**: Develop and train models in one language and execute or re-train them seamlessly across different platforms and programming environments without modification, ensuring broad compatibility.

## Getting Started

There are a couple of things that you should know about or have installed on your machine before getting started with the library and the CLI:

1. **A basic understanding of Machine Learning**: You should be familiar with ML concepts, algorithms, and model training processes. If you want to learn more about machine learning, we have blog post as well as videos going through the basics.

2. **The library imported in one of the languages supported by the framework**: To import the library simply go to your package file (e.g. `Cargo.toml` for Rust or `package.json` for JavaScript and TypScript) under *dependencies* write `jairika`, to import it in your Python project you need to install the library in your Python environment with the `pip install jairika` command and then import it in a file. If you want to use the library in your Go project, ...

3. **The CLI installed on your machine**: Alongside libraries, this framework also comes with a CLI that allows you manipulate models directly in you terminal. To install the CLI type the following command: ...

### Usage

For **Rust**, add the following to your `Cargo.toml` file:
```toml
[dependencies]
jairika = "1.0"
```

For **Python**, install the library with `pip` then import it in any python file:
```bash
$ pip install jairika
```
```py
import jairika as jrk
```

For **JavaScript**, add the following to your `package.json` file:
```json
"dependencies": {
    "jairika": "1.0"
}
```

For **Java**, ...:

For **PHP**, ...:

For **Go**, ...:

### Tutorials

-> Insert a list of videos and web pages where users can find tutorials for the framework.

## Contributing

If you are a developer that wants to participate at improving the library or the CLI, here are a couple of things you should know first.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be licensed as bellow, without any additional terms or conditions.

## License

Copyright 2024 Koseka.

This project is licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
