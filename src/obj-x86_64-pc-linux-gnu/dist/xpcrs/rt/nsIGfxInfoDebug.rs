//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIGfxInfoDebug.idl
//


/// `interface nsIGfxInfoDebug : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGfxInfoDebug {
    vtable: *const nsIGfxInfoDebugVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGfxInfoDebug.
unsafe impl XpCom for nsIGfxInfoDebug {
    const IID: nsIID = nsID(0xca7b0bc7, 0xc67c, 0x4b79,
        [0x82, 0x70, 0xed, 0x7b, 0xa0, 0x02, 0xaf, 0x08]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGfxInfoDebug {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGfxInfoDebug.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGfxInfoDebugCoerce {
    /// Cheaply cast a value of this type from a `nsIGfxInfoDebug`.
    fn coerce_from(v: &nsIGfxInfoDebug) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGfxInfoDebugCoerce for nsIGfxInfoDebug {
    #[inline]
    fn coerce_from(v: &nsIGfxInfoDebug) -> &Self {
        v
    }
}

impl nsIGfxInfoDebug {
    /// Cast this `nsIGfxInfoDebug` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGfxInfoDebugCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGfxInfoDebug {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIGfxInfoDebugCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGfxInfoDebug) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGfxInfoDebug
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGfxInfoDebugVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void spoofVendorID (in AString aVendorID); */
    pub SpoofVendorID: unsafe extern "system" fn (this: *const nsIGfxInfoDebug, aVendorID: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void spoofDeviceID (in AString aDeviceID); */
    pub SpoofDeviceID: unsafe extern "system" fn (this: *const nsIGfxInfoDebug, aDeviceID: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void spoofDriverVersion (in AString aDriverVersion); */
    pub SpoofDriverVersion: unsafe extern "system" fn (this: *const nsIGfxInfoDebug, aDriverVersion: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void spoofOSVersion (in unsigned long aVersion); */
    pub SpoofOSVersion: unsafe extern "system" fn (this: *const nsIGfxInfoDebug, aVersion: u32) -> ::nserror::nsresult,

    /* void fireTestProcess (); */
    pub FireTestProcess: unsafe extern "system" fn (this: *const nsIGfxInfoDebug) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGfxInfoDebug {


    /// `void spoofVendorID (in AString aVendorID);`
    #[inline]
    pub unsafe fn SpoofVendorID(&self, aVendorID: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SpoofVendorID)(self, aVendorID)
    }



    /// `void spoofDeviceID (in AString aDeviceID);`
    #[inline]
    pub unsafe fn SpoofDeviceID(&self, aDeviceID: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SpoofDeviceID)(self, aDeviceID)
    }



    /// `void spoofDriverVersion (in AString aDriverVersion);`
    #[inline]
    pub unsafe fn SpoofDriverVersion(&self, aDriverVersion: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SpoofDriverVersion)(self, aDriverVersion)
    }



    /// `void spoofOSVersion (in unsigned long aVersion);`
    #[inline]
    pub unsafe fn SpoofOSVersion(&self, aVersion: u32) -> ::nserror::nsresult {
        ((*self.vtable).SpoofOSVersion)(self, aVersion)
    }



    /// `void fireTestProcess ();`
    #[inline]
    pub unsafe fn FireTestProcess(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FireTestProcess)(self, )
    }


}


