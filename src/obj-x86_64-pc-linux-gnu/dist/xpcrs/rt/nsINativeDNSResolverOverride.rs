//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsINativeDNSResolverOverride.idl
//


/// `interface nsINativeDNSResolverOverride : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeDNSResolverOverride {
    vtable: *const nsINativeDNSResolverOverrideVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeDNSResolverOverride.
unsafe impl XpCom for nsINativeDNSResolverOverride {
    const IID: nsIID = nsID(0x8e38d536, 0x5501, 0x48c0,
        [0xa4, 0x12, 0x6c, 0x45, 0x00, 0x40, 0xc8, 0xc8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeDNSResolverOverride {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeDNSResolverOverride.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeDNSResolverOverrideCoerce {
    /// Cheaply cast a value of this type from a `nsINativeDNSResolverOverride`.
    fn coerce_from(v: &nsINativeDNSResolverOverride) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeDNSResolverOverrideCoerce for nsINativeDNSResolverOverride {
    #[inline]
    fn coerce_from(v: &nsINativeDNSResolverOverride) -> &Self {
        v
    }
}

impl nsINativeDNSResolverOverride {
    /// Cast this `nsINativeDNSResolverOverride` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeDNSResolverOverrideCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeDNSResolverOverride {
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
impl<T: nsISupportsCoerce> nsINativeDNSResolverOverrideCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeDNSResolverOverride) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeDNSResolverOverride
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeDNSResolverOverrideVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addIPOverride (in AUTF8String aHost, in ACString aIPLiteral); */
    pub AddIPOverride: unsafe extern "system" fn (this: *const nsINativeDNSResolverOverride, aHost: *const ::nsstring::nsACString, aIPLiteral: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setCnameOverride (in AUTF8String aHost, in ACString aCNAME); */
    pub SetCnameOverride: unsafe extern "system" fn (this: *const nsINativeDNSResolverOverride, aHost: *const ::nsstring::nsACString, aCNAME: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void clearHostOverride (in AUTF8String aHost); */
    pub ClearHostOverride: unsafe extern "system" fn (this: *const nsINativeDNSResolverOverride, aHost: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void clearOverrides (); */
    pub ClearOverrides: unsafe extern "system" fn (this: *const nsINativeDNSResolverOverride) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeDNSResolverOverride {

    /// ```text
    /// /**
    ///    * Adds an IP override for this specific host.
    ///    */
    /// ```
    ///

    /// `void addIPOverride (in AUTF8String aHost, in ACString aIPLiteral);`
    #[inline]
    pub unsafe fn AddIPOverride(&self, aHost: *const ::nsstring::nsACString, aIPLiteral: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AddIPOverride)(self, aHost, aIPLiteral)
    }


    /// ```text
    /// /**
    ///    * Sets a CNAME override for this specific host.
    ///    */
    /// ```
    ///

    /// `void setCnameOverride (in AUTF8String aHost, in ACString aCNAME);`
    #[inline]
    pub unsafe fn SetCnameOverride(&self, aHost: *const ::nsstring::nsACString, aCNAME: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCnameOverride)(self, aHost, aCNAME)
    }


    /// ```text
    /// /**
    ///    * Clears the overrides for this specific host
    ///    */
    /// ```
    ///

    /// `void clearHostOverride (in AUTF8String aHost);`
    #[inline]
    pub unsafe fn ClearHostOverride(&self, aHost: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ClearHostOverride)(self, aHost)
    }


    /// ```text
    /// /**
    ///    * Clears all the host overrides that were previously set.
    ///    */
    /// ```
    ///

    /// `void clearOverrides ();`
    #[inline]
    pub unsafe fn ClearOverrides(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearOverrides)(self, )
    }


}


