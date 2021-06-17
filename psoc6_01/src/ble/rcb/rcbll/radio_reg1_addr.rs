#[doc = "Register `RADIO_REG1_ADDR` reader"]
pub struct R(crate::R<RADIO_REG1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RADIO_REG1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RADIO_REG1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RADIO_REG1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RADIO_REG1_ADDR` writer"]
pub struct W(crate::W<RADIO_REG1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RADIO_REG1_ADDR_SPEC>;
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
impl From<crate::W<RADIO_REG1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RADIO_REG1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_ADDR` reader - N/A"]
pub struct REG_ADDR_R(crate::FieldReader<u16, u16>);
impl REG_ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_ADDR` writer - N/A"]
pub struct REG_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn reg_addr(&self) -> REG_ADDR_R {
        REG_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn reg_addr(&mut self) -> REG_ADDR_W {
        REG_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address of Register#1 in Radio (MDON)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radio_reg1_addr](index.html) module"]
pub struct RADIO_REG1_ADDR_SPEC;
impl crate::RegisterSpec for RADIO_REG1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [radio_reg1_addr::R](R) reader structure"]
impl crate::Readable for RADIO_REG1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [radio_reg1_addr::W](W) writer structure"]
impl crate::Writable for RADIO_REG1_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RADIO_REG1_ADDR to value 0x1e02"]
impl crate::Resettable for RADIO_REG1_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1e02
    }
}
