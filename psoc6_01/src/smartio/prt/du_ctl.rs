#[doc = "Register `DU_CTL` reader"]
pub struct R(crate::R<DU_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DU_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DU_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DU_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DU_CTL` writer"]
pub struct W(crate::W<DU_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DU_CTL_SPEC>;
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
impl From<crate::W<DU_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DU_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DU_SIZE` reader - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
pub struct DU_SIZE_R(crate::FieldReader<u8, u8>);
impl DU_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DU_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DU_SIZE` writer - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
pub struct DU_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DU_OPC` reader - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
pub struct DU_OPC_R(crate::FieldReader<u8, u8>);
impl DU_OPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DU_OPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_OPC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DU_OPC` writer - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
pub struct DU_OPC_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_OPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn du_size(&self) -> DU_SIZE_R {
        DU_SIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_opc(&self) -> DU_OPC_R {
        DU_OPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn du_size(&mut self) -> DU_SIZE_W {
        DU_SIZE_W { w: self }
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_opc(&mut self) -> DU_OPC_W {
        DU_OPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data unit component control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [du_ctl](index.html) module"]
pub struct DU_CTL_SPEC;
impl crate::RegisterSpec for DU_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [du_ctl::R](R) reader structure"]
impl crate::Readable for DU_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [du_ctl::W](W) writer structure"]
impl crate::Writable for DU_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DU_CTL to value 0"]
impl crate::Resettable for DU_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
