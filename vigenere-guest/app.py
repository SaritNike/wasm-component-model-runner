import os
import wit_world
from wit_world.imports import audit_log

class WitWorld(wit_world.WitWorld):
    # the environment variable is provided by our host
    keyword = os.getenv("VIGENERE_KEYWORD", "WASM").upper()
    keyword_len = len(keyword)

    def encr(self, input: str) -> str:
        audit_log.auditrecord("encrypt", f"message length '{len(input)}'")
        return self._vigenere(input, 'encrypt')

    def decr(self, input: str) -> str:  
        audit_log.auditrecord("decrypt", f"message length '{len(input)}'")
        return self._vigenere(input, 'decrypt')

    def _vigenere(self, input: str, mode: str) -> str:
        result = []
        key_index = 0
        for char in input:
            if char.isalpha():
                # subtract ord('A') to move the ASCII ord values into a range of 0 - 25
                shift = ord(self.keyword[key_index % self.keyword_len]) - ord('A')

                if mode == "decrypt":
                    shift = -shift
                
                base = ord('a') if char.islower() else ord('A')
                # get ASCII value for char, move the ASCII ord value into range of 0-25 (by subtracting the base),
                # afterwards add the base back on so you get a correct unicode character back
                new_char = chr((ord(char) - base + shift) % 26 + base)
                result.append(new_char)
                key_index += 1
            else:
                # no encoding for non-alphabetic characters
                result.append(char)
        return "".join(result)