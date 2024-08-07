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