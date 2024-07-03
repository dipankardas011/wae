from typing import override
from genai import exports
from genai.imports import outgoing_http
import json
import traceback
import os
from dotenv import load_dotenv
from termcolor import colored, cprint


class Llm(exports.Llm):
    @override
    def text_to_image(self) -> None:
        load_dotenv()

        try:
            token = os.getenv("OPENAI_API_KEY")
            if token is None:
                raise Exception("env:$OPENAI_API_KEY is not set")

            default_model = "dall-e-2"
            valid_models = ["dall-e-3", "dall-e-2"]

            model = input(f"{colored("===> Enter the model name", "cyan")} [{colored(default_model, "blue")}]: ") or default_model
            if model not in valid_models:
                raise Exception(f"Invalid model name, Valid model names are: {valid_models}")

            choice = input(colored("==> Enter the prompt: ", "cyan"))

            hd = None
            style = None
            size = ""
            if model == "dall-e-3":
                size = input(colored("==> Enter the resolution for image [1] 1024x1024 [2] 1024x1792 [3] 1792x1024 [default=1024x1024]: ", "cyan")) or "1024x1024"
                hd = input(colored("==> Enter the resolution for image [1] standard [2] hd  [default=standard]: ", "cyan")) or "standard"
                style = input(colored("==> Enter the style for image [1] natural [2] vivid [default=vivid]: ", "cyan")) or "vivid"
            else:
                size = input(colored("==> Enter the resolution for image [1] 256x256 [2] 512x512 [3] 1024x1024 [default=256x256]: ", "cyan")) or "256x256"

            text = colored("User", "yellow", attrs=["reverse", "blink"])
            print(f"\n{text}\n{colored(choice, "black")}\n")

            msg = {
                "model": model,
                "prompt": choice,
                "n": 1,
                "size": size,
            }
            if model == "dall-e-3":
                msg["quality"] = hd
                msg["style"] = style

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
                url="https://api.openai.com/v1/images/generations",
                body=str_msg.encode('utf-8'),
            )

            if http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

            data = json.loads(http_res.body)

            resp = data['data'][0]['url']
            text = colored("Assistant", "yellow", attrs=["reverse", "blink"])
            print(f"{text}\nUrl: {colored(resp, "black")}\n\n")

            http_res = outgoing_http.get_request(
                method="GET",
                headers=[],
                url=resp,
                body=None,
            )
            if http_res.status_code != 200:
                raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")
            with open("image.jpg", "wb") as f:
                f.write(http_res.body)

        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])

            print(f"{text}")
            traceback.print_exc()

    @override
    def text_to_text(self) -> None:
        load_dotenv()

        try:
            token = os.getenv("OPENAI_API_KEY")
            if token is None:
                raise Exception("env:$OPENAI_API_KEY is not set")

            default_model = "gpt-3.5-turbo"
            valid_models = ["gpt-3.5-turbo", "gpt-4-turbo", "gpt-4", "gpt-4o"]

            model = input(f"{colored("===> Enter the model name", "cyan")} [{colored(default_model, "blue")}]: ") or default_model
            if model not in valid_models:
                raise Exception(f"Invalid model name, Valid model names are: {valid_models}")

            msg = {
                "model": model,
                "messages": [
                    {"role": "system", "content": "You are a helpful assistant."}
                ]
            }
            sys_prompt = input(colored("==> Enter your system prompt: ", "cyan"))
            if len(sys_prompt) != 0:
                msg["messages"][0]["content"] = sys_prompt

            print(f"{colored('>>> For Stoping you can type \'exit\' or \'quit\' or \'stop\' or \'end\' or \'bye\'', "light_yellow")}")
            while True:
                choice = input(colored("==> Enter the prompt: ", "cyan"))
                if choice in ["exit", "quit", "stop", "end", "bye"]:
                    break

                text = colored("User", "yellow", attrs=["reverse", "blink"])
                print(f"\n{text}\n{colored(choice, "black")}\n")

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
                    raise Exception(f"StatusCode: {http_res.status_code}, Reason: {str(http_res.body)}")

                data = json.loads(http_res.body)
                resp = data['choices'][0]['message']['content']
                text = colored("Assistant", "yellow", attrs=["reverse", "blink"])
                print(f"{text}\n{colored(resp, "light_green")}\n\n")

                if data['choices'][0]['finish_reason'] != "stop":
                    cprint(f"AI stopped because of: {data['choices'][0]['finish_reason']}", "yellow", attrs=["reverse", "blink"])

                msg["messages"].append({"role": "assistant", "content": resp})

            save = input(colored("==> Do you want to save the conversation? [y/N]: ", "cyan"))
            if save.lower() in ["y", "yes", "Y"]:

                file_name = input(colored("==> Enter the file name[conversation.json]: ", "cyan")) or "conversation.json"
                with open(file_name, "w") as f:
                    f.write(f"{json.dumps(msg).encode('utf-8')}\n")

        except Exception as e:
            text = colored(f"Caught Exception: {e}", "red", attrs=["reverse", "blink"])

            print(f"{text}")
            traceback.print_exc()
