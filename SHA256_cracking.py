from pwn import *
import sys

if len(sys.argv) != 2:
    print("Invalid arguments!")
    print("Usage: {} <sha256sum>".format(sys.argv[0])) # Usage message
    exit()

wanted_hash = sys.argv[1]
password_file = "/usr/share/wordlists/rockyou.txt"
attempts = 0

with log.progress("Attempting to crack: {}!\n".format(wanted_hash)) as p: # Process started message
    with open(password_file, 'r', encoding='latin-1') as pass_list:
        for password in pass_list:
            password = password.strip("\n").encode('latin-1')
            # sha256sumhex for hash of string
            pass_hash = sha256sumhex(password) # sha256sumhex for hash of string
            p.status("[{}] {} == {}".format(attempts, password.decode('latin-1'), pass_hash))
            if pass_hash == wanted_hash:
                p.success("Password hash found after {} attempts! {} hashes to {}".format(attempts, password.decode('latin-1'), wanted_hash)) # Reponse for password found
                exit()
            attempts += 1
        p.failure("Password hash not found!") # Response for no password found
