import crypto
import string
import random
import time


class Crypto(crypto.Crypto):
    def generate_random(self, length: int) -> str:
        print(time.time())
        random.seed(1)
        lowercase = string.ascii_lowercase
        uppercase = string.ascii_uppercase
        numbers = string.digits
        special = string.punctuation
        charset = lowercase+uppercase+numbers+special
        password = ''.join(random.choice(charset) for _ in range(length))
        return password
