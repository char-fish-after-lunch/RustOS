#ifndef _RISCV_UART_H
#define _RISCV_UART_H

#include <stdint.h>

extern volatile uint32_t* uart;

#define UART_REG_RXFIFO		0
#define UART_REG_TXFIFO		1
#define UART_REG_STAT		2
#define UART_REG_CTRL		3
#define UART_REG_TXCTRL		2
#define UART_REG_RXCTRL		3
#define UART_REG_DIV		4

#define UART_TXEN		 0x1
#define UART_RXEN		 0x1

#define UART_STAT_TXFULL  0x8
#define UART_STAT_TXEMPTY 0x4
#define UART_STAT_RXFULL  0x2
#define UART_STAT_RXVALID 0x1

#define UART_CTRL_IE	  0x10
#define UART_CTRL_TXRST	  0x2
#define UART_CTRL_RXRST	  0x1

void uart_putchar(uint8_t ch);
int uart_getchar();
void query_uart(uintptr_t dtb);

#endif
