
from random import randint
from typing import List

def junk_data(amt: int, len: int) -> List[str]:

    result = []

    for i in range(amt):
        char = chr(65 + (i % 26)) 
        result.append(char * randint(1, len))

    return result

