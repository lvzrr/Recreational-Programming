rm *.key *.crypt D-ObfusKey ObfusKey
gcc ObfusKey.c -o ObfusKey -O3
gcc D-ObfusKey.c -o D-ObfusKey -O3
chmod u+s ObfusKey D-ObfusKey
