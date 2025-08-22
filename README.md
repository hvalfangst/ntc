# Norwegian Tax Calculator

A tax calculation application built with Leptos and Rust, designed to calculate Norwegian taxes for individuals, corporations (AS), and partnerships (deltakerlignet selskap) based on 2024 tax regulations.
Can be accessed in your web browser at: https://hvalfangst.github.io/norwegian-tax-calculator

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Python](https://www.python.org/downloads/) (for the `http.server` module)


## Local Development

A script named [build_and_serve](build_and_serve.sh) has been provided, which will compile and serve
the application on port 8000.

## GitHub Pages
The project is set up with a GitHub Actions Workflow script named [deploy](.github/workflows/deploy.yml), which will build and deploy the application to
GitHub Pages on pushes to main.

