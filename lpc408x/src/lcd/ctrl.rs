#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDEN` reader - LCD enable control bit. 0 = LCD disabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are low. 1 = LCD enabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are high. See LCD power-up and power-down sequence for details on LCD power sequencing."]
pub struct LCDEN_R(crate::FieldReader<bool, bool>);
impl LCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDEN` writer - LCD enable control bit. 0 = LCD disabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are low. 1 = LCD enabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are high. See LCD power-up and power-down sequence for details on LCD power sequencing."]
pub struct LCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `LCDBPP` reader - LCD bits per pixel. Selects the number of bits per LCD pixel: 000 = 1 bpp. 001 = 2 bpp. 010 = 4 bpp. 011 = 8 bpp. 100 = 16 bpp. 101 = 24 bpp (TFT panel only). 110 = 16 bpp, 5:6:5 mode. 111 = 12 bpp, 4:4:4 mode."]
pub struct LCDBPP_R(crate::FieldReader<u8, u8>);
impl LCDBPP_R {
    pub(crate) fn new(bits: u8) -> Self {
        LCDBPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBPP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDBPP` writer - LCD bits per pixel. Selects the number of bits per LCD pixel: 000 = 1 bpp. 001 = 2 bpp. 010 = 4 bpp. 011 = 8 bpp. 100 = 16 bpp. 101 = 24 bpp (TFT panel only). 110 = 16 bpp, 5:6:5 mode. 111 = 12 bpp, 4:4:4 mode."]
pub struct LCDBPP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBPP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `LCDBW` reader - STN LCD monochrome/color selection. 0 = STN LCD is color. 1 = STN LCD is monochrome. This bit has no meaning in TFT mode."]
pub struct LCDBW_R(crate::FieldReader<bool, bool>);
impl LCDBW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDBW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDBW` writer - STN LCD monochrome/color selection. 0 = STN LCD is color. 1 = STN LCD is monochrome. This bit has no meaning in TFT mode."]
pub struct LCDBW_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `LCDTFT` reader - LCD panel TFT type selection. 0 = LCD is an STN display. Use gray scaler. 1 = LCD is a TFT display. Do not use gray scaler."]
pub struct LCDTFT_R(crate::FieldReader<bool, bool>);
impl LCDTFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDTFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDTFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDTFT` writer - LCD panel TFT type selection. 0 = LCD is an STN display. Use gray scaler. 1 = LCD is a TFT display. Do not use gray scaler."]
pub struct LCDTFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDTFT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `LCDMONO8` reader - Monochrome LCD interface width. Controls whether a monochrome STN LCD uses a 4 or 8-bit parallel interface. It has no meaning in other modes and must be programmed to zero. 0 = monochrome LCD uses a 4-bit interface. 1 = monochrome LCD uses a 8-bit interface."]
pub struct LCDMONO8_R(crate::FieldReader<bool, bool>);
impl LCDMONO8_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDMONO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDMONO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDMONO8` writer - Monochrome LCD interface width. Controls whether a monochrome STN LCD uses a 4 or 8-bit parallel interface. It has no meaning in other modes and must be programmed to zero. 0 = monochrome LCD uses a 4-bit interface. 1 = monochrome LCD uses a 8-bit interface."]
pub struct LCDMONO8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDMONO8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `LCDDUAL` reader - Single or Dual LCD panel selection. STN LCD interface is: 0 = single-panel. 1 = dual-panel."]
pub struct LCDDUAL_R(crate::FieldReader<bool, bool>);
impl LCDDUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDDUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDDUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDDUAL` writer - Single or Dual LCD panel selection. STN LCD interface is: 0 = single-panel. 1 = dual-panel."]
pub struct LCDDUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDDUAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BGR` reader - Color format selection. 0 = RGB: normal output. 1 = BGR: red and blue swapped."]
pub struct BGR_R(crate::FieldReader<bool, bool>);
impl BGR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGR` writer - Color format selection. 0 = RGB: normal output. 1 = BGR: red and blue swapped."]
pub struct BGR_W<'a> {
    w: &'a mut W,
}
impl<'a> BGR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BEBO` reader - Big-endian Byte Order. Controls byte ordering in memory: 0 = little-endian byte order. 1 = big-endian byte order."]
pub struct BEBO_R(crate::FieldReader<bool, bool>);
impl BEBO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEBO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEBO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEBO` writer - Big-endian Byte Order. Controls byte ordering in memory: 0 = little-endian byte order. 1 = big-endian byte order."]
pub struct BEBO_W<'a> {
    w: &'a mut W,
}
impl<'a> BEBO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `BEPO` reader - Big-Endian Pixel Ordering. Controls pixel ordering within a byte: 0 = little-endian ordering within a byte. 1 = big-endian pixel ordering within a byte. The BEPO bit selects between little and big-endian pixel packing for 1, 2, and 4 bpp display modes, it has no effect on 8 or 16 bpp pixel formats. See Pixel serializer for more information on the data format."]
pub struct BEPO_R(crate::FieldReader<bool, bool>);
impl BEPO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEPO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEPO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEPO` writer - Big-Endian Pixel Ordering. Controls pixel ordering within a byte: 0 = little-endian ordering within a byte. 1 = big-endian pixel ordering within a byte. The BEPO bit selects between little and big-endian pixel packing for 1, 2, and 4 bpp display modes, it has no effect on 8 or 16 bpp pixel formats. See Pixel serializer for more information on the data format."]
pub struct BEPO_W<'a> {
    w: &'a mut W,
}
impl<'a> BEPO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LCDPWR` reader - LCD power enable. 0 = power not gated through to LCD panel and LCD_VD\\[23:0\\]
signals disabled, (held LOW). 1 = power gated through to LCD panel and LCD_VD\\[23:0\\]
signals enabled, (active). See LCD power-up and power-down sequence for details on LCD power sequencing."]
pub struct LCDPWR_R(crate::FieldReader<bool, bool>);
impl LCDPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDPWR` writer - LCD power enable. 0 = power not gated through to LCD panel and LCD_VD\\[23:0\\]
signals disabled, (held LOW). 1 = power gated through to LCD panel and LCD_VD\\[23:0\\]
signals enabled, (active). See LCD power-up and power-down sequence for details on LCD power sequencing."]
pub struct LCDPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDPWR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `LCDVCOMP` reader - LCD Vertical Compare Interrupt. Generate VComp interrupt at: 00 = start of vertical synchronization. 01 = start of back porch. 10 = start of active video. 11 = start of front porch."]
pub struct LCDVCOMP_R(crate::FieldReader<u8, u8>);
impl LCDVCOMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        LCDVCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDVCOMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDVCOMP` writer - LCD Vertical Compare Interrupt. Generate VComp interrupt at: 00 = start of vertical synchronization. 01 = start of back porch. 10 = start of active video. 11 = start of front porch."]
pub struct LCDVCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDVCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `WATERMARK` reader - LCD DMA FIFO watermark level. Controls when DMA requests are generated: 0 = An LCD DMA request is generated when either of the DMA FIFOs have four or more empty locations. 1 = An LCD DMA request is generated when either of the DMA FIFOs have eight or more empty locations."]
pub struct WATERMARK_R(crate::FieldReader<bool, bool>);
impl WATERMARK_R {
    pub(crate) fn new(bits: bool) -> Self {
        WATERMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WATERMARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WATERMARK` writer - LCD DMA FIFO watermark level. Controls when DMA requests are generated: 0 = An LCD DMA request is generated when either of the DMA FIFOs have four or more empty locations. 1 = An LCD DMA request is generated when either of the DMA FIFOs have eight or more empty locations."]
pub struct WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> WATERMARK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD enable control bit. 0 = LCD disabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are low. 1 = LCD enabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are high. See LCD power-up and power-down sequence for details on LCD power sequencing."]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - LCD bits per pixel. Selects the number of bits per LCD pixel: 000 = 1 bpp. 001 = 2 bpp. 010 = 4 bpp. 011 = 8 bpp. 100 = 16 bpp. 101 = 24 bpp (TFT panel only). 110 = 16 bpp, 5:6:5 mode. 111 = 12 bpp, 4:4:4 mode."]
    #[inline(always)]
    pub fn lcdbpp(&self) -> LCDBPP_R {
        LCDBPP_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection. 0 = STN LCD is color. 1 = STN LCD is monochrome. This bit has no meaning in TFT mode."]
    #[inline(always)]
    pub fn lcdbw(&self) -> LCDBW_R {
        LCDBW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD panel TFT type selection. 0 = LCD is an STN display. Use gray scaler. 1 = LCD is a TFT display. Do not use gray scaler."]
    #[inline(always)]
    pub fn lcdtft(&self) -> LCDTFT_R {
        LCDTFT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Monochrome LCD interface width. Controls whether a monochrome STN LCD uses a 4 or 8-bit parallel interface. It has no meaning in other modes and must be programmed to zero. 0 = monochrome LCD uses a 4-bit interface. 1 = monochrome LCD uses a 8-bit interface."]
    #[inline(always)]
    pub fn lcdmono8(&self) -> LCDMONO8_R {
        LCDMONO8_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection. STN LCD interface is: 0 = single-panel. 1 = dual-panel."]
    #[inline(always)]
    pub fn lcddual(&self) -> LCDDUAL_R {
        LCDDUAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Color format selection. 0 = RGB: normal output. 1 = BGR: red and blue swapped."]
    #[inline(always)]
    pub fn bgr(&self) -> BGR_R {
        BGR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Big-endian Byte Order. Controls byte ordering in memory: 0 = little-endian byte order. 1 = big-endian byte order."]
    #[inline(always)]
    pub fn bebo(&self) -> BEBO_R {
        BEBO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering. Controls pixel ordering within a byte: 0 = little-endian ordering within a byte. 1 = big-endian pixel ordering within a byte. The BEPO bit selects between little and big-endian pixel packing for 1, 2, and 4 bpp display modes, it has no effect on 8 or 16 bpp pixel formats. See Pixel serializer for more information on the data format."]
    #[inline(always)]
    pub fn bepo(&self) -> BEPO_R {
        BEPO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD power enable. 0 = power not gated through to LCD panel and LCD_VD\\[23:0\\]
signals disabled, (held LOW). 1 = power gated through to LCD panel and LCD_VD\\[23:0\\]
signals enabled, (active). See LCD power-up and power-down sequence for details on LCD power sequencing."]
    #[inline(always)]
    pub fn lcdpwr(&self) -> LCDPWR_R {
        LCDPWR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt. Generate VComp interrupt at: 00 = start of vertical synchronization. 01 = start of back porch. 10 = start of active video. 11 = start of front porch."]
    #[inline(always)]
    pub fn lcdvcomp(&self) -> LCDVCOMP_R {
        LCDVCOMP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level. Controls when DMA requests are generated: 0 = An LCD DMA request is generated when either of the DMA FIFOs have four or more empty locations. 1 = An LCD DMA request is generated when either of the DMA FIFOs have eight or more empty locations."]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD enable control bit. 0 = LCD disabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are low. 1 = LCD enabled. Signals LCD_LP, LCD_DCLK, LCD_FP, LCD_ENAB_M, and LCD_LE are high. See LCD power-up and power-down sequence for details on LCD power sequencing."]
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W {
        LCDEN_W { w: self }
    }
    #[doc = "Bits 1:3 - LCD bits per pixel. Selects the number of bits per LCD pixel: 000 = 1 bpp. 001 = 2 bpp. 010 = 4 bpp. 011 = 8 bpp. 100 = 16 bpp. 101 = 24 bpp (TFT panel only). 110 = 16 bpp, 5:6:5 mode. 111 = 12 bpp, 4:4:4 mode."]
    #[inline(always)]
    pub fn lcdbpp(&mut self) -> LCDBPP_W {
        LCDBPP_W { w: self }
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection. 0 = STN LCD is color. 1 = STN LCD is monochrome. This bit has no meaning in TFT mode."]
    #[inline(always)]
    pub fn lcdbw(&mut self) -> LCDBW_W {
        LCDBW_W { w: self }
    }
    #[doc = "Bit 5 - LCD panel TFT type selection. 0 = LCD is an STN display. Use gray scaler. 1 = LCD is a TFT display. Do not use gray scaler."]
    #[inline(always)]
    pub fn lcdtft(&mut self) -> LCDTFT_W {
        LCDTFT_W { w: self }
    }
    #[doc = "Bit 6 - Monochrome LCD interface width. Controls whether a monochrome STN LCD uses a 4 or 8-bit parallel interface. It has no meaning in other modes and must be programmed to zero. 0 = monochrome LCD uses a 4-bit interface. 1 = monochrome LCD uses a 8-bit interface."]
    #[inline(always)]
    pub fn lcdmono8(&mut self) -> LCDMONO8_W {
        LCDMONO8_W { w: self }
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection. STN LCD interface is: 0 = single-panel. 1 = dual-panel."]
    #[inline(always)]
    pub fn lcddual(&mut self) -> LCDDUAL_W {
        LCDDUAL_W { w: self }
    }
    #[doc = "Bit 8 - Color format selection. 0 = RGB: normal output. 1 = BGR: red and blue swapped."]
    #[inline(always)]
    pub fn bgr(&mut self) -> BGR_W {
        BGR_W { w: self }
    }
    #[doc = "Bit 9 - Big-endian Byte Order. Controls byte ordering in memory: 0 = little-endian byte order. 1 = big-endian byte order."]
    #[inline(always)]
    pub fn bebo(&mut self) -> BEBO_W {
        BEBO_W { w: self }
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering. Controls pixel ordering within a byte: 0 = little-endian ordering within a byte. 1 = big-endian pixel ordering within a byte. The BEPO bit selects between little and big-endian pixel packing for 1, 2, and 4 bpp display modes, it has no effect on 8 or 16 bpp pixel formats. See Pixel serializer for more information on the data format."]
    #[inline(always)]
    pub fn bepo(&mut self) -> BEPO_W {
        BEPO_W { w: self }
    }
    #[doc = "Bit 11 - LCD power enable. 0 = power not gated through to LCD panel and LCD_VD\\[23:0\\]
signals disabled, (held LOW). 1 = power gated through to LCD panel and LCD_VD\\[23:0\\]
signals enabled, (active). See LCD power-up and power-down sequence for details on LCD power sequencing."]
    #[inline(always)]
    pub fn lcdpwr(&mut self) -> LCDPWR_W {
        LCDPWR_W { w: self }
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt. Generate VComp interrupt at: 00 = start of vertical synchronization. 01 = start of back porch. 10 = start of active video. 11 = start of front porch."]
    #[inline(always)]
    pub fn lcdvcomp(&mut self) -> LCDVCOMP_W {
        LCDVCOMP_W { w: self }
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level. Controls when DMA requests are generated: 0 = An LCD DMA request is generated when either of the DMA FIFOs have four or more empty locations. 1 = An LCD DMA request is generated when either of the DMA FIFOs have eight or more empty locations."]
    #[inline(always)]
    pub fn watermark(&mut self) -> WATERMARK_W {
        WATERMARK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
