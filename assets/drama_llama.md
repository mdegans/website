[Drama LLaMA](https://github.com/mdegans/drama_llama) is a Rust library for running inferences with local language models. The name is based on my husband's nickname for the language model. In order to build this, I:

- Updated and wrapped the [low-level bindings](https://github.com/mdegans/llama-cpp-sys) to [llama.cpp](https://github.com/ggerganov/llama.cpp) to provide a safe and ergonomic wrapper.
- Rewrote the [sampling code in pure Rust](https://github.com/mdegans/drama_llama/blob/f9f92d02484c0471cc6c498a80e566af7cb2f529/src/candidates.rs#L661) in order to better understand how sampling from language models is done.
- Wrote several test binaries. One [checks language models for memorized content](https://github.com/mdegans/drama_llama/tree/main/bin/regurgitater). Another is a simple [chat app](https://github.com/mdegans/drama_llama/tree/main/bin/dittomancer).
