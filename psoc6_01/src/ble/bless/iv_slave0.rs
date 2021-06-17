#[doc = "Register `IV_SLAVE0` reader"]
pub struct R(crate::R<IV_SLAVE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV_SLAVE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV_SLAVE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV_SLAVE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV_SLAVE0` writer"]
pub struct W(crate::W<IV_SLAVE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV_SLAVE0_SPEC>;
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
impl From<crate::W<IV_SLAVE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV_SLAVE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV_SLAVE` reader - This is the IVs field, which contains the slave's portion of the initialization vector."]
pub struct IV_SLAVE_R(crate::FieldReader<u32, u32>);
impl IV_SLAVE_R {
    pub(crate) fn new(bits: u32) -> Self {
        IV_SLAVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV_SLAVE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV_SLAVE` writer - This is the IVs field, which contains the slave's portion of the initialization vector."]
pub struct IV_SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> IV_SLAVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This is the IVs field, which contains the slave's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_slave(&self) -> IV_SLAVE_R {
        IV_SLAVE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the IVs field, which contains the slave's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_slave(&mut self) -> IV_SLAVE_W {
        IV_SLAVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Initialization Vector 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_slave0](index.html) module"]
pub struct IV_SLAVE0_SPEC;
impl crate::RegisterSpec for IV_SLAVE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv_slave0::R](R) reader structure"]
impl crate::Readable for IV_SLAVE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv_slave0::W](W) writer structure"]
impl crate::Writable for IV_SLAVE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV_SLAVE0 to value 0"]
impl crate::Resettable for IV_SLAVE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
