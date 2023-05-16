# Contribution Guidelines

## Overview
This repository uses branching and merge requests to allow for code reviews before code goes into the main branch. Pushes directly to the main branch have been disabled. Do not merge to the main branch without the proper code reviews.

## Workflow
To contribute:
1. Install the Rust toolchain for your user on the server with: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Proceed with the default installation settings.
2. Run `rustup target add thumbv7em-none-eabihf` to add the board's target to the Rust toolchain.
3. (OPTIONAL) Connect to the server with SSH in VSCode.
4. (OPTIONAL) Install the `rust-analyzer` extension in VSCode.
5. Clone the repository and navigate to it.
6. Create a branch with `git checkout -b <branch_name>`. The branch name should reflect the feature that is being worked on. For example, if you are working on creating the EEPROM module, you can name the branch `eeprom`.
7. Make any changes needed, committing as you go.
8. When you are done and all code has been tested, submit the code for review by opening a merge request. Assign the merge request to yourself, and add others as reviewers. For security-critical code, ask multiple reviewers to review your code. Give the merge request a good title and detailed description so that reviewers know what feature you are working on.
9. Wait for reviewer feedback and implement any necessary changes.
10. When the review is done, merge the branch into the main branch.

## Pipeline
There is a GitLab pipeline that runs on each commit to make sure that code that is written compiles without warnings. Linter warnings will cause the pipeline to issue a warning. Build errors will cause the pipeline to fail. Commits with failed pipeline runs cannot be merged into the main branch. Re-run the pipeline if the pipeline has failed and there is an issue with the pipeline.

## Documentation
All code written for production should have good documentation and comments, including private/internal items. Use `///` for documentation comments and `//` for regular comments. All modules should be documented with `//!` at the top of the file.
