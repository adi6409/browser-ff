//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelChild.idl
//


/// `interface nsIHttpChannelChild : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpChannelChild {
    vtable: *const nsIHttpChannelChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpChannelChild.
unsafe impl XpCom for nsIHttpChannelChild {
    const IID: nsIID = nsID(0xd02b96ed, 0x2789, 0x4f42,
        [0xa2, 0x5c, 0x7a, 0xbe, 0x63, 0xde, 0x7c, 0x18]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpChannelChild {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpChannelChild.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpChannelChildCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpChannelChild`.
    fn coerce_from(v: &nsIHttpChannelChild) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpChannelChildCoerce for nsIHttpChannelChild {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelChild) -> &Self {
        v
    }
}

impl nsIHttpChannelChild {
    /// Cast this `nsIHttpChannelChild` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpChannelChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpChannelChild {
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
impl<T: nsISupportsCoerce> nsIHttpChannelChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelChild) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpChannelChild
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpChannelChildVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void addCookiesToRequest (); */
    pub AddCookiesToRequest: unsafe extern "system" fn (this: *const nsIHttpChannelChild) -> ::nserror::nsresult,

    /* [must_use] readonly attribute RequestHeaderTuples clientSetRequestHeaders; */
    /// Unable to generate binding because `native type mozilla::net::RequestHeaderTuples unsupported`
    pub GetClientSetRequestHeaders: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void GetClientSetCorsPreflightParameters (in MaybeCorsPreflightArgsRef args); */
    /// Unable to generate binding because `native type mozilla::Maybe<mozilla::net::CorsPreflightArgs> unsupported`
    pub GetClientSetCorsPreflightParameters: *const ::libc::c_void,

    /* [must_use] void removeCorsPreflightCacheEntry (in nsIURI aURI, in nsIPrincipal aRequestingPrincipal, in const_OriginAttributes aOriginAttributes); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub RemoveCorsPreflightCacheEntry: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpChannelChild {


    /// `[must_use] void addCookiesToRequest ();`
    #[inline]
    pub unsafe fn AddCookiesToRequest(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AddCookiesToRequest)(self, )
    }



    /// `[must_use] readonly attribute RequestHeaderTuples clientSetRequestHeaders;`
    const _GetClientSetRequestHeaders: () = ();


    /// `[nostdcall,notxpcom] void GetClientSetCorsPreflightParameters (in MaybeCorsPreflightArgsRef args);`
    const _GetClientSetCorsPreflightParameters: () = ();


    /// `[must_use] void removeCorsPreflightCacheEntry (in nsIURI aURI, in nsIPrincipal aRequestingPrincipal, in const_OriginAttributes aOriginAttributes);`
    const _RemoveCorsPreflightCacheEntry: () = ();

}


