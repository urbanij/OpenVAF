hello world example

.model hello hello_world
V1 p 0 DC 0
N1 p hello

.control
pre_osdi hello.osdi
tran 1m 10m
.endc

.end
