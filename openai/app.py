from typing import override
from genai import exports
from genai.imports import outgoing_http
import json
import traceback


class Llm(exports.Llm):
    @override
    def text_to_text(self, prompt: str) -> str:
        print(f"Prompt from user: {prompt}")
        return ""
