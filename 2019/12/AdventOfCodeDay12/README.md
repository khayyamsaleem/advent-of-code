# AdventOfCodeDay12

Written in swift. Currently not able to compile for Docker/Linux because Alamofire (requests library)
is restricted to macOS / Apple devices for now. Assuming you have an Apple device, you can:

1. Create a `.env` file:

```
session=<advent-of-code-session-token>
```

2. Run the code:

```bash
swift run
```

**OR**

```bash
swift build -c release
./.build/x86_64-apple-macosx/release/AdventOfCodeDay12
```
