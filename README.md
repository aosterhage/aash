# aash
`aash` (short for AAron's SHell) is a shell built by me for me to use.

It is an attempt to do many things:
- Learn the Rust programming language.
  - I am drawn to systems programming, and typically prefer solving problems in that space.
- Build something usable.
  - There are countless projects that are useful for learning but in the end you usually don't end up with something you continue to use. I am, partially, choosing to build a shell because I believe it is a piece of software that I might be able to actually use, both at home and at work. My daily shell usage is focused around a small set of features, so I believe that I could transition to using `aash` in an achievable timeframe and effort. It also remains very easy to bounce back to other shells (`bash`, `zsh`) as needed for features that I have not implemented yet.
- Have a public project.
  - Most of my software development career has been building software that is propietary, difficult to discuss, and impossible to _show_. I would like a meaningful project that anyone can inspect and discuss with me.

## Goals
- Pure Rust with no dependencies
  - I want to write all (or most) of the code. I also want `aash` to be able to run on as many systems as possible and writing it only in Rust with no (or minimal dependencies) supports that.
- Cross-platform
  - I would like for `aash` to be able to run in any environment that Cargo supports. This is probably not fully achievable and so, at a minimum, `aash` should run on MacOS and Linux, with a preference to run on any Unix-like system (e.g. FreeBSD).
- Thoroughly tested modules
  - I would like to attempt to approach building `aash` by breaking it into testable chunks and _really_ consider what tests should be included. I want the code to be organized into succint modules with tests in each that give me enough confidence to not need to execute the binary before merging.
- Follow CodeCrafters
  - CodeCrafters provides guides to build cool things by breaking them into small steps with testing to ensure your code works. I will be, minimally, referencing the [Build your own Shell](https://app.codecrafters.io/courses/shell/overview) project to help me, and potentially committing this repo's code to CodeCrafters to verify it works as they outlined.
  
### Stretch Goal
- POSIX-compliant (or a subset)
  - Shell feature-sets can vary pretty wildly and some are quite complex and time-consuming for a single developer to implement. Targetting `aash` to be POSIX-compliant gives me a clear set of features to target, while limiting the scope from being too large. Even POSIX-compliance may be too large, so its possible I only ever implement a subset.

 ## Versioning
`aash` will follow [Semantic Versioning](https://semver.org). The "public API" will be the features of the shell as written in user documentation. `aash` will start with a major version number of `0` with no clear direction on when to move to `1.0.0`.
