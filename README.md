## RustyOpenAI Library
Project that will allow to use easily OpenAI API from Rust projects.
# Things to implement
#### Text generation
##### Open AI
- [Chat completions](https://platform.openai.com/docs/guides/text-generation/chat-completions-api) - kinda done but will need some refactoring and improvements but for now is fine.
- [Moderation](https://platform.openai.com/docs/guides/moderation/overview) - is working with every call to open AI (chat completions) but will need some improvements in case of detailed comunication (what was wrong with message to AI)
- [Vision](https://platform.openai.com/docs/guides/vision)
- [Text to speech](https://platform.openai.com/docs/guides/text-to-speech) - working
- [Transcriptions](https://platform.openai.com/docs/api-reference/audio/createTranscription) - working
- [Image generation with DALL·E 3](https://platform.openai.com/docs/guides/images/introduction) - working
- [Speech to text](https://platform.openai.com/docs/guides/speech-to-text)
- [Function calling](https://platform.openai.com/docs/guides/function-calling)
- [Assistants](https://platform.openai.com/docs/assistants/overview?lang=curl)

Samples can be found in [main.rs](https://github.com/dealen/RustyOpenAI/tree/master/rusty_ai_client/src).
 
##### Project itself
- Error handling
- Loggin
- Create package from it.
