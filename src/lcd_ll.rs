/*
#define LCD_RST_PORT      GPIOF.BSRR
#define LCD_RST         12

#define LCD_DC_PORT       GPIOF.BSRR
#define LCD_DC          13

#define LCD_CS_PORT       GPIOD.BSRR
#define LCD_CS          14
*/

pub fn lcd_rst_0()
{
	//LCD_RST_PORT |= (1 << (LCD_RST + 16));
}

pub fn lcd_rst_1()
{
	//LCD_RST_PORT |= (1 << LCD_RST);
}

pub fn lcd_dc_0()
{
	//LCD_DC_PORT |= (1 << (LCD_DC + 16));
}

pub fn lcd_dc_1()
{
	//LCD_DC_PORT |= (1 << LCD_DC);
}

pub fn lcd_cs_0()
{
	//LCD_CS_PORT |= (1 << (LCD_CS + 16));
}

pub fn lcd_cs_1()
{
	//LCD_CS_PORT |= (1 << LCD_CS);
}
