from typing import override
from crypto import exports
import string
import random


class Password(exports.Password):
    @override
    def generate_random(self, length: int) -> str:
        print(f"Recieved length for generation of password: {length}")
        lowercase = string.ascii_lowercase
        uppercase = string.ascii_uppercase
        numbers = string.digits
        special = "@${}_-"
        charset = lowercase+uppercase+numbers+special
        password = ''.join(random.choice(charset) for _ in range(length))
        return password
