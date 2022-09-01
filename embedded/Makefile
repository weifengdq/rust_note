build:
	cargo build --target thumbv6m-none-eabi --release

blink:
	cd blink_stm32l052; \
	cargo-flash --chip STM32L052C8Tx --release; \
	cd ..

uart:
	cd uart_stm32l052; \
	cargo-flash --chip STM32L052C8Tx  --release; \
	cd ..

timer:
	cd timer_stm32l052; \
	cargo-flash --chip STM32L052C8Tx  --release; \
	cd ..

clean:
	cargo clean