import crypto
import string
import random


class Crypto(crypto.Crypto):
    def generate_random(self, length: int) -> str:
        lowercase = string.ascii_lowercase
        uppercase = string.ascii_uppercase
        numbers = string.digits
        special = string.punctuation
        charset = lowercase+uppercase+numbers+special
        password = ''.join(random.choice(charset) for _ in range(length))
        return password
