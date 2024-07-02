from typing import override
from genai import exports
from genai.imports import outgoing_http
import json
import traceback
import os


class Llm(exports.Llm):
    @override
    def text_to_text(self, prompt: str) -> str:
        print(f"Prompt from user: {prompt}")
        token = os.getenv("OPENAI_API_KEY")
        if token is None:
            return "env:$OPENAI_API_KEY is not set"

        try:
            msg = {
                "model": "gpt-3.5-turbo",
                "messages": [
                    {"role": "system", "content": "You are a helpful assistant."},
                    {"role": "user", "content": prompt}
                ]
            }
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
                return f"StatusCode: {http_res.status_code}, Reason: {http_res.body}"

            data = json.loads(http_res.body)
            return data['choices'][0]['message']['content']

        except Exception as e:
            print(f"Caught Exception: {e}")
            traceback.print_exc()
            return "Failed to get the response"
