all:
	cargo run -- -i hello.txt -o hello.o
	arm-none-eabi-ld -T linker.ld  -o out.elf hello.o
	qemu-system-arm -s -M virt -kernel out.elf

debug:
	gdb -ex "set architecture arm" -ex "target extended-remote :1234"  -ex "load" out.elf -ex "layout asm" -ex "layout regs" -ex "b fibonacci" -ex "b end" -ex "j fibonacci"
