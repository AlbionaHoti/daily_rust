![banner](https://user-images.githubusercontent.com/22985657/196055948-6a0f182f-80d8-480f-b738-8e5a53c1c160.png)


## 🌱 I'm currently learning

- Rust 👾

Hi, thank you for visiting the daily rust repo. The goal of this repo is to practice Rust daily from multiple resources. We're starting with two of them, the `rustling` and the Udemy course `Ultimate Rust Crash Course.`

The goal is to see how much progress someone can make if they learn and code daily in rust as a beginner. 

In the future, we can provide a hands-on course on how you can master Rust.

## Cargo

- To create a new project type `cargo new hello`
    - Cargo will create a **hello** directory with a **config** file names `Cargo.toml` and a **src** directory with a main.rs](http://main.rs)** file.
    - Config files use the toml format which stands for “Tom’s Obvious, Minimal Language”.
        - Cargo.toml is the config file for your project. It has a couple properties
            - name
            - version, rust uses **semantic versioning** which means a version number is always three numbers separated by dots, and each number has specific meaning. Learn more here: [https://semver.org](https://semver.org/).
            - authors
            - edition = “2018” if cargo didn’t use this version of rust, then you’re using an older version of rust.
            - [dependencies]
    - [main.rs](http://main.rs) file
        - `cargo run` is the command to **build** and **run** your project in one step.
        - What is `target/debug/hello`? There’s a target directory where cargo outputs all of it’s built artifacts. This is definitely a dir you want your *version control (git)* to ignore.
            - We can run the binary directory if we want: `target/debug/hello` and we’ll get the output of the project.


## Questions

1. Why is Rust good for systems programming?
    1. Rust allows you to use certain abstraction, where you don’t really pay any additional cost to using those abstractions over what you would pay if you had written the underlaying abstraction out manually instead.

## Content Creation

