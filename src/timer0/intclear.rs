#[doc = "Register `INTCLEAR` writer"]
pub type W = crate::W<IntclearSpec>;
impl core::fmt::Debug for crate::generic::Reg<IntclearSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Timer Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclearSpec;
impl crate::RegisterSpec for IntclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclear::W`](W) writer structure"]
impl crate::Writable for IntclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLEAR to value 0"]
impl crate::Resettable for IntclearSpec {
    const RESET_VALUE: u32 = 0;
}
