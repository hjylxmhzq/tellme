## "tellme" is a ChatGPT-3.5 command line client

### Installation

Download binary with proper architecture in [release page](https://github.com/hjylxmhzq/tellme/releases)

Or build from source code:

```shell
cargo build --release
```

### Usage


just run 'tellme' with any question
```shell
tellme how chatgpt works
```

if tellme was never run before, you will get the prompt as follow, then input your api token and start the chat:

```shell
tellme
> Token is not set, please set your token before starting
> (for more information, please check: https://openai.com/blog/introducing-chatgpt-and-whisper-apis)
> [input your api token]
# then start your chat
```

or you can just run with --reset to remove the token and any other config

```shell
tellme --reset
```

#### Use as REPL

if you run 'tellme' without any argument, it will run in REPL mode instead of ask->answer->exit mode

```shell
tellme
>> ask one question
[some answer]
>> and another question
[another answer]
```
![sample](https://raw.githubusercontent.com/hjylxmhzq/tellme/main/sample.gif)

### Proxy

set HTTPS_PROXY or https_proxy as the proxy target

```shell
export https_proxy=http://127.0.0.1:1234
```
