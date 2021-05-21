//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/glean/xpcom/nsIFOG.idl
//


/// `interface nsIFOG : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFOG {
    vtable: *const nsIFOGVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFOG.
unsafe impl XpCom for nsIFOG {
    const IID: nsIID = nsID(0x98d0e975, 0x9cad, 0x4ce3,
        [0xae, 0x2f, 0xf8, 0x78, 0xb8, 0xbe, 0x63, 0x07]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFOG {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFOG.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFOGCoerce {
    /// Cheaply cast a value of this type from a `nsIFOG`.
    fn coerce_from(v: &nsIFOG) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFOGCoerce for nsIFOG {
    #[inline]
    fn coerce_from(v: &nsIFOG) -> &Self {
        v
    }
}

impl nsIFOG {
    /// Cast this `nsIFOG` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFOGCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFOG {
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
impl<T: nsISupportsCoerce> nsIFOGCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFOG) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFOG
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFOGVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void initializeFOG (); */
    pub InitializeFOG: unsafe extern "system" fn (this: *const nsIFOG) -> ::nserror::nsresult,

    /* void setLogPings (in boolean aEnableLogPings); */
    pub SetLogPings: unsafe extern "system" fn (this: *const nsIFOG, aEnableLogPings: bool) -> ::nserror::nsresult,

    /* void setTagPings (in ACString aDebugTag); */
    pub SetTagPings: unsafe extern "system" fn (this: *const nsIFOG, aDebugTag: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void sendPing (in ACString aPingName); */
    pub SendPing: unsafe extern "system" fn (this: *const nsIFOG, aPingName: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFOG {

    /// ```text
    /// /**
    ///    * Initialize FOG.
    ///    *
    ///    * To be scheduled at some opportune time after the bulk of Firefox startup
    ///    * has completed.
    ///    */
    /// ```
    ///

    /// `void initializeFOG ();`
    #[inline]
    pub unsafe fn InitializeFOG(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).InitializeFOG)(self, )
    }


    /// ```text
    /// /**
    ///    * Enable or Disable the logging of pings in the Glean SDK.
    ///    * See https://firefox-source-docs.mozilla.org/toolkit/components/glean/testing.html
    ///    * for details.
    ///    *
    ///    * @param aEnableLogPings - true to enable logging, false to disable.
    ///    */
    /// ```
    ///

    /// `void setLogPings (in boolean aEnableLogPings);`
    #[inline]
    pub unsafe fn SetLogPings(&self, aEnableLogPings: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetLogPings)(self, aEnableLogPings)
    }


    /// ```text
    /// /**
    ///    * Set the tag to be applied to pings assembled from now on.
    ///    * See https://firefox-source-docs.mozilla.org/toolkit/components/glean/testing.html
    ///    * for details.
    ///    *
    ///    * @param aDebugTag - The string tag to apply.
    ///    *                    If it cannot be applied (e.g it contains characters that are
        ///    *                    forbidden in HTTP headers) the old value will remain.
    ///    */
    /// ```
    ///

    /// `void setTagPings (in ACString aDebugTag);`
    #[inline]
    pub unsafe fn SetTagPings(&self, aDebugTag: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetTagPings)(self, aDebugTag)
    }


    /// ```text
    /// /**
    ///    * Send the named ping.
    ///    * See https://firefox-source-docs.mozilla.org/toolkit/components/glean/testing.html
    ///    * for details.
    ///    *
    ///    * @param aPingName - The name of the ping to send. If no ping of that name
    ///    *                    exists, or the ping is known but cannot be assembled
    ///    *                    (e.g if it is empty), no ping will be sent.
    ///    */
    /// ```
    ///

    /// `void sendPing (in ACString aPingName);`
    #[inline]
    pub unsafe fn SendPing(&self, aPingName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SendPing)(self, aPingName)
    }


}


