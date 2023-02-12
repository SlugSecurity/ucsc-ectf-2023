# Contribution Guidelines

## Overview
This repository uses branching and merge requests to allow for code reviews before code goes into the main branch. Pushes directly to the main branch have been disabled. Do not merge to the main branch without the proper code reviews.

## Workflow
To contribute:
1. Get access to the server on Tailscale. Ask on Discord if you do not have access.
2. Create a user on the server and setup Git access on there.
3. Clone the repository and navigate to it.
4. Create a branch with `git checkout -b <branch_name>`. The branch name should reflect the feature that is being worked on. For example, if you are working on creating the EEPROM module, you can name the branch `eeprom`.
5. Make any changes needed, committing as you go.
6. When you are done and all code has been tested, submit the code for review by opening a merge request. Assign the merge request to yourself, and add others as reviewers. For security-critical code, ask multiple reviewers to review your code. Give the merge request a good title and detailed description so that reviewers know what feature you are working on.
7. Wait for reviewer feedback and implement any necessary changes.
8. When the review is done, merge the branch into the main branch.

## Pipeline
There is a pipeline that runs on each commit to make sure that code that is written compiles without warnings. Linter warnings will cause the pipeline to issue a warning. Build errors will cause the pipeline to fail. Commits with failed pipeline runs cannot be merged into the main branch. Re-run the pipeline if the pipeline has failed and there is an issue with the pipeline.

## Documentation
All code written for production should have good documentation and comments, including private/internal items. Use `///` for documentation comments and `//` for regular comments. All modules should be documented with `//!` at the top of the file.

To access the public and private documentation from the latest commit to the main branch, make sure you have access to Tailscale. Then, go to http://plantmachineectf.tail25abc.ts.net/.
