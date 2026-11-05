# exchat

A small Rust + [egui](https://github.com/emilk/egui) desktop chat app with a built-in research loop.

You type a prompt. A model refines it, searches the web, checks whether the results are enough, and searches again if not. Once there is enough evidence, a model writes the answer. You can keep chatting from there.

- Bring your own providers: any OpenAI-compatible API (name, base URL, key, multiple models each)
- Pick which model does which job (refine, judge, explain, title)
- Settings and API keys persist between runs
- History list of past chats, auto-titled by a fast model

**Status:** scaffolding. See [PLAN.md](PLAN.md) for goals, architecture and milestones.

## Run

```sh
cargo run
```

Currently loads (or creates defaults for) the settings file and prints a summary. The egui window lands in milestone 1.
