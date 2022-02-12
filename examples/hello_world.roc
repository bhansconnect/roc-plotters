app "hello_world"
    packages { pf: "../platform" }
    imports []
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/hello_world.svg",
        title: "Hello, World!",
        subtitle: "This data is coming from Roc:",
        width: 1024,
        height: 768,
        points1: [{ x: -1, y: 1}, { x: 0, y: -1}, { x: 1, y: 1}],
        points2: [{ x: -1, y: -2}, { x: 0, y: 2}, { x: 1, y: -2}],
    }
