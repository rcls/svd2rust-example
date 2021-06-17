#[doc = "Register `ADV_ACCADDR_L` reader"]
pub struct R(crate::R<ADV_ACCADDR_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_ACCADDR_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_ACCADDR_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_ACCADDR_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_ACCADDR_L` writer"]
pub struct W(crate::W<ADV_ACCADDR_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_ACCADDR_L_SPEC>;
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
impl From<crate::W<ADV_ACCADDR_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_ACCADDR_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_ACCADDR_L` reader - Lower 16 bit of ADV packet access code"]
pub struct ADV_ACCADDR_L_R(crate::FieldReader<u16, u16>);
impl ADV_ACCADDR_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADV_ACCADDR_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_ACCADDR_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_ACCADDR_L` writer - Lower 16 bit of ADV packet access code"]
pub struct ADV_ACCADDR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_ACCADDR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_l(&self) -> ADV_ACCADDR_L_R {
        ADV_ACCADDR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_l(&mut self) -> ADV_ACCADDR_L_W {
        ADV_ACCADDR_L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV packet access code low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_accaddr_l](index.html) module"]
pub struct ADV_ACCADDR_L_SPEC;
impl crate::RegisterSpec for ADV_ACCADDR_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_accaddr_l::R](R) reader structure"]
impl crate::Readable for ADV_ACCADDR_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_accaddr_l::W](W) writer structure"]
impl crate::Writable for ADV_ACCADDR_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_ACCADDR_L to value 0xbed6"]
impl crate::Resettable for ADV_ACCADDR_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbed6
    }
}
