## Python 101
### Variables and Data Types
```python
# lists []
name_list = ["neut", "235FT"] 
# Can assign varibles to parts of your list
name1, name2 = name_list

# Tuple ()
name_tuple = ("neut", "235FT") 

# Dictionary {}
name_dictionary = {"neut": 4, "235FT": 5}

# Boolean | True or False
name_boolean = False

# Range
name_range = range(6)

# bytes
name_bytes = b"neut2"
```
### Numbers
```python
# Numbers
t1_int = 1 # int 
t1_float = 1.0 # float
t1_complex = 3.14j # complex numbers
t1_hex = 0xa # Prints as 10 (You can use any number system)
t1_octal = 0o10 # Prints as 8
# Python can even add these different number types together

print(abs(4))
print(abs(-4)) # We can also get abslute values 
print(round(8.4)) # We can also round numbers (.5 is rounded down)
print(bin(8)) # Can print out the binary value of a number
(print(hex(8)) # Can print out the hex of a number
```
### String Formatting
```python
string1 = "I am a string!"
string2 = 'I am a string too!' # Double and single quotes doesn't make a different in this case
string3 = """I am a long
long
string""" # Many lines for one string (prints this way as well)

string4 = "I\"m a string" # Using a string with quotes within it (Escape the character with \" 
string5 = 'I\'m a string' # Works with single quotes as well
# Can also use different quotes within the variable vs to define the string to avoid escaping characters

string6 = "Hi \n new line!" # Can also escape to a new line with n 
string7 = "\\ \x42" # Escaping backslashes and hex

string8 = "a" * 10 # Print something out a given number of times (10) 
print(len(string8)) # Print out the variable length of a string (10)

print("string" in string4) # (True) prints out wether text is within a string T/F
print(string.startsWith("I")) # T/F See what letter the sting starts with

print(string4.inxes("string")) # (6) Because string is in index 6 (Will be talked about more in the lists section

print(string4.upper()) # Print in uppercase
print(string4.lower()) # Print in lowercase

messy_string = "  Messy string!    "
print(messey_string.strip()) # Remove messy spaces
print(messy_string.replace("!", "?")) # Replace ! with ? 
print(messy_string.replace("!", "?")).strip()) # Stack both strip and replace together
print(messy_string.split()) # Split the string into a list (each word is one item)
print(messy_string.split(",")) # Split on something other then spaces, in this case commas

print(string4.encode())
print(string4.encode("utf-8")) # Encoding strings

print(string4.rjust(25)) # Right justified to be a length of 25 (adds spaces to the right until you get to the right string length or cuts the string to be shorter))
print(string4.rjust(25, "x")) # Adds x instead of space
print(string4.ljust(25)) # Ajusts from the left 

# Strings cannot be changed after created, so we would need to make a new string
print("I am "+ "a string") # Combinds the together to make a whole new string
print("String 4 is " + len(string4) + "characters long") # Cannot add int to string here
print("String 4 is " + str(len(string4) + " characters long") # Cast the int into a string to make it work

print(1 + 1) # Adds them into 2 (does not combine them like strings)
print("1" + "1") # Combinds them into 11 because they are strings not ints
print(type("1" + "1")) 

print("string4 is {} characters long!".format(len(string4))) # Use a placeholder {} and add to it later

print("{} {} {}".format(len(string4), 5.0, 0x12)) # the format method can print out all of the data types like this
print("{0} {2} {1}".format(len(string4), 5.0, 0x12)) # choose the order to print the items out in
print("{length}".format(length=len(string4))) # another way to specify the order

length = len(string4) 
print(f"string4 is {length} characters long") # Make a formated string that you can place varaibles within

print("string4 is {length:.2f} chracters long".format(length=len(string4))) # Print it out as a float (or choose whatever type you want)
# 2f = float with 2 decimal places 
# x = hex, b = binary, o = octal 
```
### Booleans & Operators
```python
# True or False values
valid = True
not_valid = False 

print(valid == True) # See if valid is true
print(not_valid == False) # See not_valid is false
print(not_valid != True) # See not_valid does not equal true 
print(not valid) # See is not_valid is not True (False output)
print(not not_valid) # See is not_valid is not True (True output) 

print((10 < 9) == True) # False
print((10 == 10) == True) # True
print((10 != 10) == True) # False
print((10 >= 10) == True) # True
print((10 <= 10) == True) # True
print((10 > 9) == True) # True

# We don't need the == True part below is also valid
print((10 < 9)) # False
print((10 == 10)) # True
print((10 != 10)) # False
print((10 >= 10)) # True
print((10 <= 10)) # True
print((10 > 9)) # True

print(10 > 5 and 10 < 5) # Both need to be true to return True (because of and)
print(10 > 5 or 10 < 5) # One needs to be to true to return True (because of or)

# 0 is False and 1 is True 

# MATH
print(10 + 10) # Addtion
print(10 - 10) # Subtraction
print(10 * 10) # Multiplication 
print(10 / 10) # Division
print(10 // 10) # Rounds to the nearest whole number
print(10 ** 10) # 10 to the power of ten
print(10 % 10) # Divides the left by the right and returns the remainder 

x = 10
print(x) # 10
x = x + 1
print(x) # 11 
x += 1 # same as x = x + 1 (Works with all operators)
print(x) # 11

x = 13
print(bin(x)) # 0b1101 (binary of 13)
print(bin(x)[2:].rjust(4, "0")) # Makes the output more readable (returns 1101) 

y = 5
print(bin(y)[2:].rjust(4, "0")) # Makes the output more readable (returns 0101)

# Bitwise comparisons 
print(bin(x & y)[2:].rjust(4, "0")) # Outputs 0101 (5)
# Checks if both bits are set to 1 and if they are then it returns 1 in that place, if not it returns 0 in that place

# Can do that the same thing with or instead of and
print(bin(x | y)[2:].rjust(4, "0"))

# Bit shifts
print(bin(x >> 1)[2:].rjust(4, "0") # Moved all the bits across one places (to the left)
print(bin(x >> 2)[2:].rjust(4, "0") # Moved all the bits across two places
print(bin(x << 2)[2:].rjust(4, "0") # Shifts in the other direction two places
```
### Tuples
```python
# Can store multiple items in a single varaibles (cannot be changed)
tuple_items = ("item1", "item2", "item2") # Making a tuple 
print(tuple_items)

tuple_numbers = (1, 2, 3) # Doesn't have to be strings 

tuple_repeat = ('Combine!',) * 4
print(tuple_repeat) # Prints Combine! four times 

mixed_tuple = ("A", 1, ("A", 1)) # Contains a string, a number and a nestled tuple

# Tuples can be combined but not changed
tuple_combined = tuple_items + tuple_numbers
print(tuple_combined)

# Can take things from a tuple and make them into varaibles
item1, item2, item3 = tuple_items
print(item1)
print(item2)
print(item3) # Will print one on each line

print("item2" in tuple_items) # Returns true because it is within the tuple

# Get the index of a item within the tuple
print(tuple_items.index("item2")) # Returns 2 (because we count from 0)

# Print based on idex
print(tuple_items[0])
print(tuple_items[1])
print(tuple_items[2])

print(len(tuple_items)) # Print out the length of the tuple

print(tuple_items[-1]) # Print starting from the end (the last item)
print(tuple_items[-2]) # Print starting from the end (the second last item)

# Slices | extract a number of elements from the tuple up to, but not including the end of the slice
print(tuple_items[0:2]) # item1 and item2 (0 and 1 items) 
print(tuple_items[:2]) # same as the above
print(tuple_items[-3:-1]) # extract from the end

string1 = "I am a string!"
print(string1[0:4]) # Prints I am
print(string1[:-1]) # Prints everything except the last character (!) 
```
### Lists
```python
# We can change the data once it has been defined with lists
list1 = ["A", "B", "C", "D", "E", "F"]
print(list1) # Prints them all out 

# Can use all of the datatypes within a list (even more lists or empty lists)

# Can be indexed just like tuples
print(list1[0]) # First item
print(list1[-1]) # Last item

# Can index nested lists as well
print(list2[3][0]) # The first item from the 4th item within list2 

# Can change lists
list1[0] = X # Index 0 is now the value of X 
del list1[0] # Delete index 0
list1.insert(0, "A") # Insert A into index 0 
list1 = ["A"] + list1 # Add to list1
list1.append("G") # Can append to list if we do not care about the index (or want to add to the end)

# Values
print(max(list1)) # Maximum value
print(min(list1)) # Minimum value 

# Index
print(list1.index("C")) # What is the index of C in list1? 

list1.reverse() # Reverse the list
list1 = list1[::-1] # Rereverse the list 

print(list1.count("A")) # Count the number of A's in the list (2) 

list1.pop() # Remove the last item in a list

# Extend a list
list3 = ["H", "I", "J", ]
list.extend(list3) # Add them together

list1.clear() # Remove everything from a list

# Sorting
list4 = [8, 12, 5, 6, 17, 2]
list4.sort() # Sort by numerical value (asending order)
list4.sort(reverse=True) # sort in the reverse order (desending order)

# Cannot copy a list with list2 = list1 (any changes to list2 would effect list1 in this case)
list5 = list4.copy() # To actually copy data and have them operate seperatly 

# Map function is used to apply some function to all items in the list (works with things other than lists)
list7 = ["1", "2", "3",]
list8 = list(map(float, list7)) # Turns everything in list7 to floats stored within list8
```
### Dictionaries
```python
# Store data values in key:value pairs {}
dict1 = {"a":1, "b":2, "c":3} # Key first then value 
print(dict1["a"]) # Specify the key to print out the value (Can't index like with lists)
print(dict1.get("a")) # Get a's value in a different way

print(dict1.keys()) # See all of the keys
print(dict1.values()) # See all of the values
print(dict1.items()) # See all of the items

# Can't have duplicates for the keys 
dict1["d"] = 4 # Can add new items though
dict1["a"] = 0 # Can change the values as well
dict1.update({"a: 1"}) # Can change the values this way as well
dict1.pop("d") # Remove from the dictionary
del dict1["c"] # Remove from the dictionary

dict1["c"] = {"a":1, "b";2} # Updates the dictionary to have the key of c and the value of another dictionary

# Can also make empty dictionaries
dict2 = {}
dict2 = dict{} 
```
### Sets
```python
# Multiple values in one variable {}
set1 = {"a", "b", "c"} # No key value pair like a dictionary
print(set1) # Prints in a random order because sets do not have a set order

# Cannot use index with sets because they are not ordered
# Cannot have duplicate values
# Can be made up of any datatypes

# Can new items to sets
set1.add("d")
set3 = set(("b", 2, False))
set1.update(set3) # Add set3 to set1

# Combine a list and a set together
list1 = ["a", "b", "c"]
set4 = {4, 5, 6}
set4.update(list1)

set5 = {4, 5, 6}
set6 = set4.union(set5) # An union of two sets

# Remove items from sets
set4.remove(1) # Remove a given value in this case 1 (must have the value within the set or else it will error out)
set4.discard(1) # Remove items without erroring out if the item is not within the set

# Can use pop with sets but since sets are not ordered this can give unintended results
set1.pop() 
```
### Conditionals
```python
# Make different decision depending on the outcome of comparisons evaluating to either True or False

if True:
	print("True") # Print True if True 

if False:
	print("False") # Print False if False (doesn't print because the conditional is not true)

if not False:
	print("Not false") # Not false would be true 

if 1 < 1:
	print("1 < 1") # Can add conditionals to these if statements

if 1 < 1:
	print("1 < 1")
elif 1 <= 1
	print("1 <= 1") # Add multiple conditionals 
else:
	print("else 1") # Print is nothing else is true

# Prints only one true statement
if 1 < 1:
	print("1 < 1")
elif 1 <= 1
	print("1 <= 1") # Will print the first true statements it sees
elif 2 <= 2
	print("2 <= 2") # Never reaches this statement
else:
	print("else reached") # Don't need this statement here, but if nothing is true then nothing will print

# Multiple things need to be true (can be more than 2 things)
if 1 > 0 and 0 < 1:
	print("1 > 0 and 0 < 1")


# Only one thing needs to be true (can be more than 2 things)
if 1 > 0 or 0 < 1:
	print("1 > 0 or 0 < 1")

# Combinding "and" and "or" together
if (1 > 0 or 0 < 1) and 1 == 0:
	print("1 > 0 or 0 < 1 and 1 == 0")

# Combinding multiple or statements (works with and as well)
if (1 > 0 or 0 < 1) or 1 == 0:
	print("1 > 0 or 0 < 1 or 1 == 0")
	print("printing 1 > 0 or 0 < 1 or 1 == 0 again") # Can have many things here that will run if the conditional is met, but the spacing needs to be consistent) 

if 0 < 1: print("0 < 1") # Can do it all in one line

# In line turnery (can be used instead of a elif or else block)
print("1 >= 1") if 1 >= 1 else print("1 < 1")
print("1") if 0 > 1 else print("2") if 0 < 1 else print("3")
```
### Loops
```python
a = 1
print(a) 
a += 1
print(a) 
a += 1
print(a)

# Instead of manually adding to a over and over again we can loop it for a number of times 

# While loop will execute until something is true
a = 1
while a < 5:
	a += 1
	print(a) # Prints 2, 3, 4, 5 

# For loop will execute a certain number of times
for i in [0, 1, 2, 3, 4]; # Each of these values are temporaily "i"
	print(1+6) # They then get 6 added to them and are printed here

# Can make use of range so we don't have to write everything out manually
for i in range(5):
	print(1+6) # Same results as above 

# Nested loops
for i in range(3):
	for j in range(3):
		print(i,j) # Will print i in the left collum and j in the right collum (See image below for that)

# Loop control statements 
for i in range(5):
	if i == 2:
		break # Ends after this thing happens 
	print(i) # I will iterate up to 2 while printing it out and then break because of the break statement

for i in range(5):
	if i == 2:
		continue # Will skip back up to start of the loop and continue iterating (So it will print 0, 1, 3, 4 in this case (skipping 2)) 
	print(i)

for i in range(5):
	if i == 2:
		pass # prints 0, 1, 2, 3, 4 (nothing changes in this case)
	print(i)

# Looping over strings instead of numbers 
for c in "string":
	print(c) # Prints each character in the string each on their own line (iterates over them each once and then runs the print statement for each) 

# Looping over dictionaries
for key, value in {"a":1, "b":2, "c":3}.items():
	print(key, value) # Print out each key value pair each on a given line 
```
### Reading and Writing Files
```python
f = open('top-100.txt') # Open and read a txt file 

f = open('top-100.txt', 'rt' ) # Open and read a txt file
print(f.read()) # Print out each line within the file

f = open('top-100.txt', 'rt' ) # Open and read a txt file
print(f.readlines()) # print outs an array of each line from the file 

f.seek(0) # Returns file position back to the start so you can read it again 
print(f.readlines())

f.seek(0)
for line in f:
	print(line.strip()) # Print out each line and strip any extra characters
f.close() # Close the file

f open("test.txt", "w")
f.write("Test line!") # write to a file (Will overwrite whatever is already there)
f.close()

f open("test.txt", "a") # Append mode to add to a file that is already there
f.write("Test line!")
f.close()

# get file info
print(f.name)
print(f.closed)
print(f.mode)

with open('rockyou.txt') as f # gives us an error because of encoding with some of the data
	for line in f:
		pass 


with open('rockyou.txt', encoding='latin-1') as f # Specify the encoding and now we can read it
	for line in f:
		pass
```
### User Input
```python
test = input()
print(test) # prints input only after user enters some input

test = input("Enter the IP:") # Print the message to the screen and then have the user enter a value (much more clear)
print(test)


while True: # Create a loop that will last forever (because it is always true)
  test = input("Enter IP: ")
  print(">>> {}".format(test)) # Print the IP
  if test == "exit": # If the user enters exit
    break # End the loop
  else:
    print("exploiting..") # If the user does not type exit
```
### Exceptions and Error Handling
```python
# Python runs all checks as the lines are ran 
# We can handle these errors with trycatch block

# Indentation error
print(1)
 print(2)

try:
  f = open("doesnotexistfilename") # Get a file not found error (without the exception)
except:
  print("The file does not exist!") # prints a custom error (even if there is another with the code) 

# Specify the errors you are trying to catch
try:
  f = open("randomfile")
except FileNotFoundError: # Print just for a certain type of error, i.e file not found 
  print("File does not exist")
except Exception as e:
  print(e)
finally:
  print("This always get printed") # Always get printed no matter the error or lack thereof 

# Can also raise exceptions
n = 100
if n == 0:
  raise Exception("n cannot be 0!") # Make an error if something you define happens or doesn't happen (in this case if n is 0) 
if type(n) is not int:
  raise Exception("n must be an integer") # Make an error if n is not int 
print(1/n)

# assertions similar to raise exception, program will stop executing of assertions are raised | hard error checking 
n = 1
assert(n != 0)
print(1/n) #triggers AssertionError if n is 0
```
### Comprehensions
```python
# Make a list based on some iterable 

list1 = ['a', 'b', 'c'] # Normal list

#list comprehensions
list2 = [x for x in list1] # iterating over each element in list1 and adding it to list2 

list3 = [x for x in list1 if x == 'a'] # iterating over each element in list1 and adding it to list3 IF that item is 'a' 

list4 = [x for x in range(5)] # iterating over numbers adding it to list4 for a range of 5 (0-4)

list5 = [hex(x) for x in range(5)] # iterating over numbers with each value being in hex from in the new list

list6 = [hex(x) if x > 0 else "X" for x in range(5)] # adding a turnary which makes applies the hex function to anything greater the 0 and prints X for anything less than 0

list7 = [x for x in range(5) if x == 0 or x == 1] # Only print out 0 and 1 because they were the only ones that evaluated to be true 

list8 = [[1,2,3],[4,5,6],[7,8,9]] # Lists in a list | Nested lists 

list9 = [y for x in list8 for y in x] # [1,2,3,4,5,6,7,8,9] (prints out all of the elements of the nested lists)

set1 = {x * x for x in range(5)} # Add each element in the range to itself 

list10 = [c for c in "stringtext"] # Takes each character in the string makes it into a list 

texthere = "".join(list10) # Combind items in an iterable into a single string 
texthere = "-".join(list10) # Can add a delimiter between each character 
```
### Functions and Code Reuse
```python
# Function run only when they are called (don't need to write them multiple times) 

# define function
def function1():
  print("hello from function!")

# call function
function1()

# returns a value
def function2():
  return "hello from function2!" # Doesn't print even when called, but it just returns the value

func2 = function2() # The return value is now applied to this variable 
print(func2) # print it out

# accepts parameters
def function3(s):
  print("\t{}".format(s))
function3("param") # Prints out "param" with a tab before it | paramaters are not optional in this function

def function4(s1, s2): # Miltiple arguments
  print("{} {}".format(s1,s2))
function4("check","this")
function4(s1="check",s2="this") # Can specifically assign each parameter to a given variable when calling it to change the default order

# set a default value 
def function5(s1 = "default"): # Used if no argument is passed 
  print(s1)

# for any number of argument | unknown number 
def function6(s1, *more):
  print("{} {}".format(s1, " ".join([s for s in more])))
function6("func6", "arg1", "arg2", "arg3")

# dictionary of arguments | can iterate over each item in the dictionary and print out the key:value pairs
def function7(**ks): 
  for a in ks:
    print(a, ks[a])
function7(a="1", b="2", c="3")

def function8(s, f, i, l): # Can use any type of input

# global scope and function scope differ (local scope = function scope)

v = 100 # global scope

def function9():
  global v   # specify global scoped variable
  v += 1 # increment by 1
  print(v)

function9()
print(v)

# functions can call other functions
def function10():
  function1()

# recursion | make sure you add a condition that caused them to terminate at some point
def function11(x):
  print(x)
  if x > 0: # only perform the function if x > 0 | exit condition 
    function11(x-1)

function11(5)

# Recursions can usually just be written in an iterative way as well 
def function12(x):
	while x >= 0:
	print(x)
	x -= 1
```
### Lambdas
```python
# Single line anonymous function (no name) 

add_4 = lambda x : x + 4
print(add_4(4)) # Prints 8

add = lambda x, y : x + y 
print(add(10,4)) # Prints 14

print((lambda x, y : x + y)(10,4)) # Defined and evaulated all in one line 

is_even = lambda x : x % 2 == 0 # Prints true/false based on rather the provided number is even 
print(is_even(2)) # true 

blocks = lambda x, y : [x[i:i+y] for i in range(0, len(x), y)]
print(blocks("string", 2)) # ['st', 'ri', 'ng']

to_ord = lambda x : [ord(i) for i in x]
print(to_ord("ABCD")) # [65, 66, 67, 68] returns the int representation of each character 
```
## Extending Python
### Python Package Manager
```python
pip install pwntools

pip list # view installed libraries

pip freeze # view libraries and version in a format that is easier for pip to read (these are what you would put in your requirements.txt files and the likes

pip install -r requirements.txt # install from requirements.txt
```
### Virtual Enviroments
```python
pip install virtualenv # Installation
 
mkdir virtual-demo # Make directory

cd virtual-demo

python3 -m venv env # Start virtual env

# Activate virtual env
source env/bin/activate # Prompt includes 'env' now

python3 # By default this env does not include any of our normal modules

# In virtual env, check which python binary is being used
which python3 # Different from /usr/bin/python3 which is our main python installation

pip install pwntools # Install package in virtual env

deactivate # Deactivate virtual env
```
### Intro to sys
```python
import sys
import time

print(sys.version) # outputs the version of Python interpreter

print(sys.executable) # output the Python binary used


print(sys.platform) # output is linux


for line in sys.stdin: # takes the input and prints it, until "exit"
  if line.strip() == "exit":
    break
  sys.stdout.write(">> {}".format(line))

for i in range(1,5):
  sys.stdout.write(str(i))
  sys.stdout.flush()  # Clears the internal buffer of file

# Used to simulate a progress bar
for i in range(0,51):
  time.sleep(0)
  sys.stdout.write("{} [{}{}]\r".format(i, '#'*i, "."*(50 - i)))
  sys.stdout.flush()
sys.stdout.write("\n")

print(sys.argv) # lists the arguments supplied to script | first name will be the name of script every time

if len(sys.argv) != 3:
  print("[X] To run {} enter username and password".format(sys.argv[0]))
  sys.exit(5) # exit with particular exit code

username = sys.argv[1]
password = sys.argv[2]

# Access the path for modules
print(sys.path)

# list the modules
print(sys.modules)

# exit with certain exit code
sys.exit(0)
```
### Intro to Requests
```python
import requests

x = requests.get('http://httpbin.org/get')

print(x.headers)
print(x.headers['Server'])
print(x.status_code)
print(x.elapsed) # time elapsed
print(x.cookies)
print(x.content) # output in the form of bytes
print(x.text) # output in the form of unicode

x = requests.get('http://httpbin.org/get', params={'id':'1'})
print(x.url)

x = requests.get('httpL//httpbin.org/get?id=2')
print(x.url)

x = requests.get('http://httpbin.org/get', params={'id':'3'}, headers={'Accept':'application/json', 'test_header':'test'}) # prints the response in json format
print(x.text)

x = requests.delete('http://httpbin.org/delete')
print(x.text)

x = requests.post('http://httpbin.org/post', data={'a':'b'})
print(x.text)

files = {'file': open('google.jpg', 'rb')}
x = requests.post('http://httpbin.org/post', files=files)
print(x.text)

x = requests.get('http://httpbin.org/get', auth=('username','password')) # basic auth
print(x.text)

x = requests.get('https://expired.badssl.com', verify=False) # gives SSL error unless 'verify=False'

x = requests.get('http://github.com', allow_redirects=False)
print(x.headers)

x = requests.get('http://httpbin.org/get'. timeout=0.01)
print(x.content)

x = requests.get('http://httpbin.org/cookies', cookies={'a':'b'}) # sessions and cookies
print(x.content)

x = requests.Session()
x.cookies.update({'a':'b'})
print(x.get('http://httpbin.org/cookies').text)

x = requests.get('https://api.github.com/events')
print(x.json())

x = requests.get('https://www.google.com/images/googlelogo.png') # get images
with open('google2.png', 'wb') as f:
  f.write(x.content)
```
### Intro to Pwntools
```python
from pwn import *

print(cyclic(50))
print(cyclic_find("laaa"))
# example functions for buffer overflow

print(shellcraft.sh())

p = process("/bin/sh") # start the local process
p.sendline("echo hello;")
p.interactive() # interactive session

r = remote("127.0.0.1", 1234)
r.sendline("hello")
r.interactive()
r.close()

# packing and unpacking
print(p32(0x13371337))
print(hex(u32(p32(0x13371337))))

l = ELF('/bin/bash') # load files
print(hex(l.address))
print(hex(l.entry))
print(hex(l.got['write']))
print(hex(l.plt['write']))

for address in l.search(b'/bin/sh\x00'):
  print(hex(address))

print(hex(next(l.search(asm('jmp esp')))))

# encoding, hashing
print(xor("A", "B"))
print(b64e(b"test"))
print(md5sumhex(b"hello"))

# low level functions
print(bits(b'a'))
```
