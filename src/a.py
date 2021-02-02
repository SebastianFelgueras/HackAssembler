JUMP_MNEMONIC_TO_BITS = {
        None : '000',
        'JGT': '001',
        'JEQ': '010',
        'JGE': '011',
        'JLT': '100',
        'JNE': '101',
        'JLE': '110',
        'JMP': '111'
    }
for a in JUMP_MNEMONIC_TO_BITS:
    print(f"(\"{a}\",\"{JUMP_MNEMONIC_TO_BITS[a]}\"),")