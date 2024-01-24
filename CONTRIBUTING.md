# Any Help is Awesome!

Remember, no contribution is too small! Whether it's fixing a typo, improving documentation, or adding new features, every contribution is valuable. If you encounter issues or have questions, don't hesitate to open an issue on the GitHub repository.

Thank you for contributing to TReq! 🚀

# Steps...
1. Fork and Develop: Start by forking the TReq repository, and create a new branch for your contribution. Develop and make the changes in your branch.
2. Check Guidelines: Before submitting your contribution, ensure that it aligns with the project's guidelines. This includes coding standards, documentation conventions, and any other relevant guidelines. Run always `cargo fmt` to ensure the code is formatted.
3. Run Tests: If your contribution involves code changes or introduces new features, run the tests to ensure everything is working as expected.
```sh
cargo test
```

You'll need to run the E2E tests too, they need to be in a clear enviroment of TReq usage setup (like folders it creates for usage)
```sh
cargo test --features run_e2e_tests
```

Docker can be used to ensure a clean environment for E2E tests:
```sh
docker compose -f tests/compose.yml build run_e2e 
docker compose -f tests/compose.yml run --rm run_e2e
```
4. Submit a Pull Request: Once you are confident in your changes, submit a pull request. Provide a clear and detailed description of your contribution.
5. Wait for Feedback: After submitting your pull request, wait for feedback from maintainers. They may review your changes, provide suggestions, or request modifications.

