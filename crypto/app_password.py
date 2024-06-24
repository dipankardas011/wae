from password_guest import password
import string
import random
import time


class Password(password.Password):
    def generate_random(self, length: int) -> str:
        lowercase = string.ascii_lowercase
        uppercase = string.ascii_uppercase
        numbers = string.digits
        special = string.punctuation
        charset = lowercase+uppercase+numbers+special
        password = ''.join(random.choice(charset) for _ in range(length))
        return f"{password} {time.time()}"
