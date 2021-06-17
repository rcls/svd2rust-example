#[doc = "Register `MATRIXARB` reader"]
pub struct R(crate::R<MATRIXARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIXARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIXARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIXARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIXARB` writer"]
pub struct W(crate::W<MATRIXARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIXARB_SPEC>;
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
impl From<crate::W<MATRIXARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIXARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_ICODE` reader - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
pub struct PRI_ICODE_R(crate::FieldReader<u8, u8>);
impl PRI_ICODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_ICODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_ICODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_ICODE` writer - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
pub struct PRI_ICODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ICODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PRI_DCODE` reader - D-Code bus priority."]
pub struct PRI_DCODE_R(crate::FieldReader<u8, u8>);
impl PRI_DCODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_DCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_DCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_DCODE` writer - D-Code bus priority."]
pub struct PRI_DCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_DCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PRI_SYS` reader - System bus priority."]
pub struct PRI_SYS_R(crate::FieldReader<u8, u8>);
impl PRI_SYS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_SYS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_SYS` writer - System bus priority."]
pub struct PRI_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PRI_GPDMA` reader - General Purpose DMA controller priority."]
pub struct PRI_GPDMA_R(crate::FieldReader<u8, u8>);
impl PRI_GPDMA_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_GPDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_GPDMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_GPDMA` writer - General Purpose DMA controller priority."]
pub struct PRI_GPDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_GPDMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PRI_ETH` reader - Ethernet DMA priority."]
pub struct PRI_ETH_R(crate::FieldReader<u8, u8>);
impl PRI_ETH_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_ETH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_ETH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_ETH` writer - Ethernet DMA priority."]
pub struct PRI_ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ETH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `PRI_LCD` reader - LCD DMA priority."]
pub struct PRI_LCD_R(crate::FieldReader<u8, u8>);
impl PRI_LCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_LCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_LCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_LCD` writer - LCD DMA priority."]
pub struct PRI_LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_LCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `PRI_USB` reader - USB DMA priority."]
pub struct PRI_USB_R(crate::FieldReader<u8, u8>);
impl PRI_USB_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_USB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_USB` writer - USB DMA priority."]
pub struct PRI_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ROM_LAT` reader - ROM latency select. Should always be 0."]
pub struct ROM_LAT_R(crate::FieldReader<bool, bool>);
impl ROM_LAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROM_LAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_LAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_LAT` writer - ROM latency select. Should always be 0."]
pub struct ROM_LAT_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_LAT_W<'a> {
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
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline(always)]
    pub fn pri_icode(&self) -> PRI_ICODE_R {
        PRI_ICODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&self) -> PRI_DCODE_R {
        PRI_DCODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&self) -> PRI_SYS_R {
        PRI_SYS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline(always)]
    pub fn pri_gpdma(&self) -> PRI_GPDMA_R {
        PRI_GPDMA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&self) -> PRI_ETH_R {
        PRI_ETH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&self) -> PRI_LCD_R {
        PRI_LCD_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline(always)]
    pub fn pri_usb(&self) -> PRI_USB_R {
        PRI_USB_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline(always)]
    pub fn rom_lat(&self) -> ROM_LAT_R {
        ROM_LAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline(always)]
    pub fn pri_icode(&mut self) -> PRI_ICODE_W {
        PRI_ICODE_W { w: self }
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&mut self) -> PRI_DCODE_W {
        PRI_DCODE_W { w: self }
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&mut self) -> PRI_SYS_W {
        PRI_SYS_W { w: self }
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline(always)]
    pub fn pri_gpdma(&mut self) -> PRI_GPDMA_W {
        PRI_GPDMA_W { w: self }
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&mut self) -> PRI_ETH_W {
        PRI_ETH_W { w: self }
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&mut self) -> PRI_LCD_W {
        PRI_LCD_W { w: self }
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline(always)]
    pub fn pri_usb(&mut self) -> PRI_USB_W {
        PRI_USB_W { w: self }
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline(always)]
    pub fn rom_lat(&mut self) -> ROM_LAT_W {
        ROM_LAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Matrix arbitration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrixarb](index.html) module"]
pub struct MATRIXARB_SPEC;
impl crate::RegisterSpec for MATRIXARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrixarb::R](R) reader structure"]
impl crate::Readable for MATRIXARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrixarb::W](W) writer structure"]
impl crate::Writable for MATRIXARB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATRIXARB to value 0"]
impl crate::Resettable for MATRIXARB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
