#[doc = "Register `ARB_CFG` reader"]
pub struct R(crate::R<ARB_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_CFG` writer"]
pub struct W(crate::W<ARB_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_CFG_SPEC>;
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
impl From<crate::W<ARB_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTO_MEM` reader - Enables Auto Memory Configuration. Manual memory configuration by default."]
pub struct AUTO_MEM_R(crate::FieldReader<bool, bool>);
impl AUTO_MEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_MEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_MEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_MEM` writer - Enables Auto Memory Configuration. Manual memory configuration by default."]
pub struct AUTO_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_MEM_W<'a> {
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
#[doc = "DMA Access Configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_CFG_A {
    #[doc = "0: No DMA"]
    DMA_NONE = 0,
    #[doc = "1: Manual DMA"]
    DMA_MANUAL = 1,
    #[doc = "2: Auto DMA"]
    DMA_AUTO = 2,
}
impl From<DMA_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMA_CFG` reader - DMA Access Configuration."]
pub struct DMA_CFG_R(crate::FieldReader<u8, DMA_CFG_A>);
impl DMA_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMA_CFG_A> {
        match self.bits {
            0 => Some(DMA_CFG_A::DMA_NONE),
            1 => Some(DMA_CFG_A::DMA_MANUAL),
            2 => Some(DMA_CFG_A::DMA_AUTO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_NONE`"]
    #[inline(always)]
    pub fn is_dma_none(&self) -> bool {
        **self == DMA_CFG_A::DMA_NONE
    }
    #[doc = "Checks if the value of the field is `DMA_MANUAL`"]
    #[inline(always)]
    pub fn is_dma_manual(&self) -> bool {
        **self == DMA_CFG_A::DMA_MANUAL
    }
    #[doc = "Checks if the value of the field is `DMA_AUTO`"]
    #[inline(always)]
    pub fn is_dma_auto(&self) -> bool {
        **self == DMA_CFG_A::DMA_AUTO
    }
}
impl core::ops::Deref for DMA_CFG_R {
    type Target = crate::FieldReader<u8, DMA_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_CFG` writer - DMA Access Configuration."]
pub struct DMA_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No DMA"]
    #[inline(always)]
    pub fn dma_none(self) -> &'a mut W {
        self.variant(DMA_CFG_A::DMA_NONE)
    }
    #[doc = "Manual DMA"]
    #[inline(always)]
    pub fn dma_manual(self) -> &'a mut W {
        self.variant(DMA_CFG_A::DMA_MANUAL)
    }
    #[doc = "Auto DMA"]
    #[inline(always)]
    pub fn dma_auto(self) -> &'a mut W {
        self.variant(DMA_CFG_A::DMA_AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `CFG_CMP` reader - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
pub struct CFG_CMP_R(crate::FieldReader<bool, bool>);
impl CFG_CMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_CMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_CMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_CMP` writer - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
pub struct CFG_CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_CMP_W<'a> {
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
impl R {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub fn auto_mem(&self) -> AUTO_MEM_R {
        AUTO_MEM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    pub fn dma_cfg(&self) -> DMA_CFG_R {
        DMA_CFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub fn cfg_cmp(&self) -> CFG_CMP_R {
        CFG_CMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub fn auto_mem(&mut self) -> AUTO_MEM_W {
        AUTO_MEM_W { w: self }
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    pub fn dma_cfg(&mut self) -> DMA_CFG_W {
        DMA_CFG_W { w: self }
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub fn cfg_cmp(&mut self) -> CFG_CMP_W {
        CFG_CMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arbiter Configuration Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_cfg](index.html) module"]
pub struct ARB_CFG_SPEC;
impl crate::RegisterSpec for ARB_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_cfg::R](R) reader structure"]
impl crate::Readable for ARB_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_cfg::W](W) writer structure"]
impl crate::Writable for ARB_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_CFG to value 0"]
impl crate::Resettable for ARB_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
