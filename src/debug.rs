use embedded_graphics::{
  mono_font::{MonoFont, MonoTextStyleBuilder},
  pixelcolor::Rgb565,
  prelude::*,
  text::{Alignment, Baseline, LineHeight, Text, TextStyleBuilder},
};
use pico::{
  hal::uart::{Enabled, UartPeripheral},
  pac::UART0,
  Screen,
};

pub type ArrayString = arrayvec::ArrayString<{ NCHARS as usize }>;

pub const SCREEN_SIZE: u16 = 240;
pub const SCREEN_SIZE2: usize = SCREEN_SIZE as usize * SCREEN_SIZE as usize;

static mut SCREEN: Option<Screen> = None;

static mut UART: Option<UartPeripheral<Enabled, UART0>> = None;

#[allow(dead_code)]
fn usage() {
  //let led = pins.led.into_push_pull_output();

  //debug::init_debug(led, explorer.screen);
  //let mut string = debug::ArrayString::new();

  //let string = debug::breakup(string);
  //debug::sprint(&string);
}

pub fn screen<'a>() -> &'a Screen {
  unsafe { SCREEN.as_ref().unwrap() }
}
pub fn screen_mut<'a>() -> &'a mut Screen {
  unsafe { SCREEN.as_mut().unwrap() }
}

pub fn uart<'a>() -> &'a UartPeripheral<Enabled, UART0> {
  unsafe { UART.as_ref().unwrap() }
}
pub fn uart_mut<'a>() -> &'a mut UartPeripheral<Enabled, UART0> {
  unsafe { UART.as_mut().unwrap() }
}

pub fn init_debug(uart: UartPeripheral<Enabled, UART0>) {
  unsafe {
    UART = Some(uart);
  }
}

const FONT: MonoFont = embedded_graphics::mono_font::ascii::FONT_4X6;

const CHAR_WIDTH: u32 = FONT.character_size.width + FONT.character_spacing;
const CHAR_HEIGHT: u32 = FONT.character_size.height;
const CHARS_PER_ROW: u32 = SCREEN_SIZE as u32 / CHAR_WIDTH;
const CHARS_PER_COL: u32 = SCREEN_SIZE as u32 / CHAR_HEIGHT;
const NCHARS: u32 = CHARS_PER_ROW * CHARS_PER_COL;

pub fn breakup(s: ArrayString) -> ArrayString {
  let mut r = ArrayString::new();
  let mut last = 0usize;
  for (i, c) in s.char_indices() {
    r.push(c);
    if c == '\n' {
      last = i;
    } else if i - last >= CHARS_PER_ROW as usize {
      r.push('\n');
      last = i;
    }
  }
  r
}

pub fn sprint(text: &str) {
  screen_mut().clear(Rgb565::BLACK).unwrap();
  draw_text(text);
}

pub fn draw_text(text: &str) {
  let char_style = MonoTextStyleBuilder::new()
    .font(&FONT)
    .text_color(Rgb565::GREEN)
    .background_color(Rgb565::BLACK)
    //.reset_background_color()
    .build();

  let text_style = TextStyleBuilder::new()
    .alignment(Alignment::Left)
    .baseline(Baseline::Top)
    .line_height(LineHeight::Pixels(6))
    .build();

  Text::with_text_style(text, Point::new(0, 0), char_style, text_style)
    .draw(screen_mut())
    .unwrap();
}

use core::{fmt::Write, panic::PanicInfo};
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  let mut buf = ArrayString::new();
  writeln!(buf, "{}", info).unwrap();
  buf = breakup(buf);
  uart().write_full_blocking(buf.as_bytes());

  //screen_mut().clear(Rgb565::RED).unwrap();
  //draw_text(&buf);

  loop {}
}
