#[doc = "Register `EPDMAST` reader"]
pub struct R(crate::R<EPDMAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPDMAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPDMAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPDMAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP_DMA_ST0` reader - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit must be 0)."]
pub struct EP_DMA_ST0_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST1` reader - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
pub struct EP_DMA_ST1_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST2` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST2_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST3` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST3_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST4` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST4_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST5` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST5_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST6` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST6_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST7` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST7_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST8` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST8_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST9` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST9_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST10` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST10_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST11` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST11_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST12` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST12_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST13` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST13_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST14` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST14_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST15` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST15_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST16` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST16_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST17` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST17_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST18` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST18_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST19` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST19_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST20` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST20_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST21` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST21_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST22` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST22_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST23` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST23_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST24` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST24_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST24_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST25` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST25_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST25_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST26` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST26_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST26_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST27` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST27_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST27_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST28` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST28_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST28_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST29` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST29_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST29_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST30` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST30_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST30_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_DMA_ST31` reader - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
pub struct EP_DMA_ST31_R(crate::FieldReader<bool, bool>);
impl EP_DMA_ST31_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_DMA_ST31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DMA_ST31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    pub fn ep_dma_st0(&self) -> EP_DMA_ST0_R {
        EP_DMA_ST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    pub fn ep_dma_st1(&self) -> EP_DMA_ST1_R {
        EP_DMA_ST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st2(&self) -> EP_DMA_ST2_R {
        EP_DMA_ST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st3(&self) -> EP_DMA_ST3_R {
        EP_DMA_ST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st4(&self) -> EP_DMA_ST4_R {
        EP_DMA_ST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st5(&self) -> EP_DMA_ST5_R {
        EP_DMA_ST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st6(&self) -> EP_DMA_ST6_R {
        EP_DMA_ST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st7(&self) -> EP_DMA_ST7_R {
        EP_DMA_ST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st8(&self) -> EP_DMA_ST8_R {
        EP_DMA_ST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st9(&self) -> EP_DMA_ST9_R {
        EP_DMA_ST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st10(&self) -> EP_DMA_ST10_R {
        EP_DMA_ST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st11(&self) -> EP_DMA_ST11_R {
        EP_DMA_ST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st12(&self) -> EP_DMA_ST12_R {
        EP_DMA_ST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st13(&self) -> EP_DMA_ST13_R {
        EP_DMA_ST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st14(&self) -> EP_DMA_ST14_R {
        EP_DMA_ST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st15(&self) -> EP_DMA_ST15_R {
        EP_DMA_ST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st16(&self) -> EP_DMA_ST16_R {
        EP_DMA_ST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st17(&self) -> EP_DMA_ST17_R {
        EP_DMA_ST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st18(&self) -> EP_DMA_ST18_R {
        EP_DMA_ST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st19(&self) -> EP_DMA_ST19_R {
        EP_DMA_ST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st20(&self) -> EP_DMA_ST20_R {
        EP_DMA_ST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st21(&self) -> EP_DMA_ST21_R {
        EP_DMA_ST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st22(&self) -> EP_DMA_ST22_R {
        EP_DMA_ST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st23(&self) -> EP_DMA_ST23_R {
        EP_DMA_ST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st24(&self) -> EP_DMA_ST24_R {
        EP_DMA_ST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st25(&self) -> EP_DMA_ST25_R {
        EP_DMA_ST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st26(&self) -> EP_DMA_ST26_R {
        EP_DMA_ST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st27(&self) -> EP_DMA_ST27_R {
        EP_DMA_ST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st28(&self) -> EP_DMA_ST28_R {
        EP_DMA_ST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st29(&self) -> EP_DMA_ST29_R {
        EP_DMA_ST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st30(&self) -> EP_DMA_ST30_R {
        EP_DMA_ST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st31(&self) -> EP_DMA_ST31_R {
        EP_DMA_ST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "USB Endpoint DMA Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epdmast](index.html) module"]
pub struct EPDMAST_SPEC;
impl crate::RegisterSpec for EPDMAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epdmast::R](R) reader structure"]
impl crate::Readable for EPDMAST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPDMAST to value 0"]
impl crate::Resettable for EPDMAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
