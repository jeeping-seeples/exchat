# exchat plan

## Goal

A minimal desktop chat app where one prompt triggers a small research loop (refine → search → judge → search more → explain) and then behaves like a normal chat for follow-ups. Small codebase, few dependencies, no server.

## Non-goals (for now)

- No plugin system, no agents with arbitrary tools. Search is the only tool.
- No accounts, sync or cloud storage. Everything is local.
- No mobile or web build.
- No provider-specific APIs. Everything speaks the OpenAI-style `POST {base_url}/chat/completions` format.

## User-facing features

1. **Chat view**: input box, message list, streaming answer, visible research steps (queries run, rounds used) in a collapsible section.
2. **Settings button** (window or side panel):
   - Providers: add/edit/remove. Each has a name, base URL, API key, and a list of model IDs (multiple per provider).
   - Roles: choose a provider + model for each job.
     - `refiner`: rewrites the prompt into a standalone question and proposes search queries
     - `judge`: reads gathered evidence and decides enough / not enough, proposing follow-up queries
     - `explainer`: writes the final answer and handles follow-up chat
     - `titler`: fast, cheap model that names new chats
   - Max search rounds (default 4).
3. **History button**: list of past chats (title + date), click to reopen, delete.
4. **Persistence**: settings, API keys and chats survive restarts.

## Research loop

```
user prompt
   |
   v
[refiner]  -> { needs_search, standalone_question, queries[] }
   |
   +-- needs_search = false -----------------------------+
   |                                                      |
   v                                                      |
[search] run queries, collect {title, url, snippet}       |
   |                                                      |
   v                                                      |
[judge]  -> { enough, missing, next_queries[] }           |
   |   enough = false and rounds < max: back to [search]  |
   v                                                      |
[explainer] streams answer, cites sources by number  <----+
   |
   v
(first exchange only) [titler] -> chat title
```

Notes:
- Refiner and judge return strict JSON. Parse defensively and fall back to "search with the raw prompt" / "enough" on bad output so the loop always terminates.
- Follow-up messages go through the refiner too, so "explain that simpler" skips search but "what about X?" can trigger one.
- Hard cap on rounds (`max_search_rounds`) is the only stop condition the model cannot override.
- Sources are numbered and stored with the message so citations stay valid when the chat is reopened.

## Architecture

```
src/
  main.rs        entry point, eframe setup
  config.rs      Settings, Provider, ModelRef, Roles + load/save   (done)
  llm.rs         OpenAI-compatible client: chat + streaming       (M2)
  search.rs      SearchBackend trait + one implementation         (M3)
  pipeline.rs    research loop, runs off the UI thread            (M3)
  history.rs     Chat/Message types + on-disk store               (M4)
  ui/            chat view, settings window, history panel        (M1+)
```

- **Threading**: UI thread never blocks. Pipeline runs on a background thread (tokio runtime) and sends events (`Step`, `Token`, `Done`, `Error`) over a channel; UI drains the channel each frame and calls `ctx.request_repaint()`.
- **HTTP**: `reqwest` with rustls, SSE parsing for streamed tokens.
- **Persistence**: `directories` for per-OS paths.
  - Settings: `settings.json` in the config dir (written atomically, `0600` on unix).
  - Chats: one JSON file per chat in the data dir, plus an index for the history list. Move to SQLite only if this gets slow.
- **API keys**: plaintext in `settings.json` for v1, never logged, never in git. Optional upgrade: OS keyring.

## Settings file shape

```json
{
  "providers": [
    {
      "name": "example",
      "base_url": "https://api.example.com/v1",
      "api_key": "...",
      "models": ["model-a", "model-b"]
    }
  ],
  "roles": {
    "refiner":   { "provider": "example", "model": "model-a" },
    "judge":     { "provider": "example", "model": "model-a" },
    "explainer": { "provider": "example", "model": "model-b" },
    "titler":    { "provider": "example", "model": "model-a" }
  },
  "max_search_rounds": 4
}
```

## Milestones

- [x] **M0** Repo, plan, settings model with persistence + tests
- [ ] **M1** egui window: chat view with fake echo reply, settings window editing providers/models/roles and saving
- [ ] **M2** `llm.rs`: real chat completions + streaming against a configured provider; explainer role works as a plain chat
- [ ] **M3** `search.rs` + `pipeline.rs`: refine → search → judge loop with visible steps and citations
- [ ] **M4** `history.rs`: save chats, history panel, reopen/delete, titler role
- [ ] **M5** Polish: error toasts, cancel button, markdown rendering, CI (fmt, clippy, test), release builds

## Open decisions

- **Search backend.** Not chosen yet. Candidates: Brave Search API, Tavily, a self-hosted SearXNG. The trait boundary keeps this swappable. Needs one decision before M3, including where its API key lives (likely a `search` block in settings).
- **Page fetching.** Snippets only, or also fetch and trim page text for the judge/explainer? Snippets-only is simpler; fetching gives better answers.
- **Markdown rendering** in the chat view (`egui_commonmark`) vs plain text at first.
- **Egui version.** `eframe` moves fast between releases (latest on crates.io is 0.36.x); M1 should start from the current official `eframe_template` and pin whatever `cargo add eframe` resolves.
