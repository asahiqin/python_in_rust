# Python in Rust
### A python interpreter written in rust

## Feature

We support syntax for partial Python 3.10.  
What is implemented?  
- [x] Basic Variable Operation  
- [x] Condition Statement  
- [x] While Statement
- [ ] For Statement.  
- [ ] Function(including 'def' and 'lambda')  
- [ ] Class  
- [x] A temporary output statement

Make sure you switch the master bench,you can find test about running a python file in src/test.rs


Developer bench is rewriteing the logic of namespace,so it cannot run anything

### Example 
src/test_py/test.py
```python
a = 1
b = 1
c = "hello world"
print __name__
if a==1:
    print 1
else:
    print 2
while a < 10:
    a = a+1
    if a == 3:
        continue
    print a
    b = b * a
else:
    print c
print b
```
`print <Statment>`is a temporary output statement


## Test
```bash
git clone https://github.com/asahiqin/python_in_rust.git
cd python_in_rust
cargo test py_test --nocapture
```

## About

### License
This repo use GPL 3.0 license.To be concise,you can edit the code and republish it,what you need is just annotating the original repository

### Original intention
Alright,we just want to take part in the Chinese Primary and Secondary School Student Programming Competition.  
So it's not a project that can use in a production environment.  
You can say this is a boring project because we repeat making wheels.  
However, it's a challanging thing to make a interpreter,isn't it?