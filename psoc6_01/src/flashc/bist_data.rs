#[doc = "Register `BIST_DATA[%s]` reader"]
pub struct R(crate::R<BIST_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_DATA[%s]` writer"]
pub struct W(crate::W<BIST_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_DATA_SPEC>;
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
impl From<crate::W<BIST_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - BIST data register to store the expected value for data comparison. For a 128-bit Flash memory, there will be 4 BIST_DATA registers to store 128-bit value."]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - BIST data register to store the expected value for data comparison. For a 128-bit Flash memory, there will be 4 BIST_DATA registers to store 128-bit value."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BIST data register to store the expected value for data comparison. For a 128-bit Flash memory, there will be 4 BIST_DATA registers to store 128-bit value."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST data register to store the expected value for data comparison. For a 128-bit Flash memory, there will be 4 BIST_DATA registers to store 128-bit value."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST data register(s)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_data](index.html) module"]
pub struct BIST_DATA_SPEC;
impl crate::RegisterSpec for BIST_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_data::R](R) reader structure"]
impl crate::Readable for BIST_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_data::W](W) writer structure"]
impl crate::Writable for BIST_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_DATA[%s]
to value 0"]
impl crate::Resettable for BIST_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
