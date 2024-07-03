from typing import override
from genai import exports
from genai.imports import outgoing_http
import json
import traceback
import os
from dotenv import load_dotenv
import logging
import coloredlogs

coloredlogs.install()

logger = logging.getLogger(__name__)

class Llm(exports.Llm):
    @override
    def text_to_image(self, prompt: str) -> str:
        ...

    @override
    def text_to_text(self) -> None:
        load_dotenv()
        token = os.getenv("OPENAI_API_KEY")
        if token is None:
            logger.error("env:$OPENAI_API_KEY is not set")
            return

        try:
            model = "gpt-3.5-turbo"
            valid_models = ["gpt-3.5-turbo", "gpt-4-turbo", "gpt-4", "gpt-4o"]
            model = input(f"==> Enter the model name [gpt-3.5-turbo]: ") or model
            if model not in valid_models:
                raise Exception(f"Invalid model name, Valid model names are: {valid_models}")

            msg = {
                "model": model,
                "messages": [
                    {"role": "system", "content": "You are a helpful assistant."}
                ]
            }
            sys_prompt = input("==> Enter your system prompt: ")
            if len(sys_prompt) != 0:
                msg["messages"][0]["content"] = sys_prompt

            logger.info(">>> For Stoping you can type 'exit' or 'quit' or 'stop' or 'end' or 'bye'")
            while True:
                choice = input("==> Enter the prompt: ")
                if choice in ["exit", "quit", "stop", "end", "bye"]:
                    break

                msg["messages"].append({"role": "user", "content": choice})

                str_msg = json.dumps(msg)

                http_res = outgoing_http.get_request(
                    method="POST",
                    headers=[
                        outgoing_http.RequestHeader(
                            key="Content-Type", value="application/json",
                        ),
                        outgoing_http.RequestHeader(
                            key="Authorization", value=f"Bearer {token}",
                        ),
                    ],
                    url="https://api.openai.com/v1/chat/completions",
                    body=str_msg.encode('utf-8'),
                )

                if http_res.status_code != 200:
                    raise Exception(f"StatusCode: {http_res.status_code}, Reason: {http_res.body}")

                data = json.loads(http_res.body)
                resp = data['choices'][0]['message']['content']
                logger.info(f"Response from AI [STOPPED:<{data['choices'][0]['finish_reason']}>]\n{resp}\n====\n")
                msg["messages"].append({"role": "assistant", "content": resp})

            save = input("==> Do you want to save the conversation? [y/N]: ")
            if save.lower() in ["y", "yes", "Y"]:
                file_name = input("Enter the file name: ")
                with open(file_name, "w") as f:
                    f.write(f"{json.dumps(msg).encode('utf-8')}\n")

        except Exception as e:
            logger.error(f"Caught Exception: {e}")
            traceback.print_exc()
