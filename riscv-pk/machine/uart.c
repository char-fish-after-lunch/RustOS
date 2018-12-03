#include <string.h>
#include "uart.h"
#include "fdt.h"

volatile uint32_t* uart;
volatile uint32_t* uart_prm;

void uart_putchar(uint8_t ch)
{
  #ifdef __BOARD_zedboard
    int32_t stat = uart[UART_REG_STAT];
    while (stat & UART_STAT_TXFULL) {
      stat = uart[UART_REG_STAT];
    }
    uart[UART_REG_TXFIFO] = ch;
  #else
    #ifdef __riscv_atomic
        int32_t r;
        do {
          __asm__ __volatile__ (
            "amoor.w %0, %2, %1\n"
            : "=r" (r), "+A" (uart[UART_REG_TXFIFO])
            : "r" (ch));
        } while (r < 0);
    #else
        volatile uint32_t *tx = uart + UART_REG_TXFIFO;
        while ((int32_t)(*tx) < 0);
        *tx = ch;
    #endif
  #endif    
}

int uart_getchar()
{
  #ifdef __BOARD_zedboard
    int32_t stat = uart[UART_REG_STAT];
    if (stat & UART_STAT_RXVALID) {
      return uart[UART_REG_RXFIFO];
    } else {
      return -1;
    }
  #else
    int32_t ch = uart[UART_REG_RXFIFO];
    if (ch < 0) return -1;
    return ch;
  #endif
}

#ifdef __BOARD_zedboard
void uart_prm_putchar(uint8_t ch)
{
  int32_t stat = uart_prm[UART_REG_STAT];
  while (stat & UART_STAT_TXFULL) {
    stat = uart_prm[UART_REG_STAT];
  }
  uart_prm[UART_REG_TXFIFO] = ch;
}

int uart_prm_getchar()
{
  int32_t stat = uart_prm[UART_REG_STAT];
  if (stat & UART_STAT_RXVALID) {
    return uart_prm[UART_REG_RXFIFO];
  } else {
    return -1;
  }
}
#endif

struct uart_scan
{
  int compat;
  uint64_t reg;
};

static void uart_open(const struct fdt_scan_node *node, void *extra)
{
  struct uart_scan *scan = (struct uart_scan *)extra;
  memset(scan, 0, sizeof(*scan));
}

static void uart_prop(const struct fdt_scan_prop *prop, void *extra)
{
  struct uart_scan *scan = (struct uart_scan *)extra;
  if (!strcmp(prop->name, "compatible") && !strcmp((const char*)prop->value, "sifive,uart0")) {
    scan->compat = 1;
  } else if (!strcmp(prop->name, "reg")) {
    fdt_get_address(prop->node->parent, prop->value, &scan->reg);
  }
}

static void uart_done(const struct fdt_scan_node *node, void *extra)
{
  struct uart_scan *scan = (struct uart_scan *)extra;
  #ifdef __BOARD_zedboard
    uart = (void*)(uintptr_t)0x60000000;
    uart[UART_REG_CTRL] = UART_CTRL_TXRST | UART_CTRL_RXRST;
    uart[UART_REG_CTRL] = UART_CTRL_IE;

    uart_prm = (void*)(uintptr_t)0x60001000;
    uart_prm[UART_REG_CTRL] = UART_CTRL_TXRST | UART_CTRL_RXRST;
    uart_prm[UART_REG_CTRL] = UART_CTRL_IE;
  #else
    if (!scan->compat || !scan->reg || uart) return;

    // Enable Rx/Tx channels
    uart = (void*)(uintptr_t)scan->reg;
    uart[UART_REG_TXCTRL] = UART_TXEN;
    uart[UART_REG_RXCTRL] = UART_RXEN;
  #endif
}

void query_uart(uintptr_t fdt)
{
  struct fdt_cb cb;
  struct uart_scan scan;

  memset(&cb, 0, sizeof(cb));
  cb.open = uart_open;
  cb.prop = uart_prop;
  cb.done = uart_done;
  cb.extra = &scan;

  fdt_scan(fdt, &cb);
}
