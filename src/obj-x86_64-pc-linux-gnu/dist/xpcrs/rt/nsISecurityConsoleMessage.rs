//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsISecurityConsoleMessage.idl
//


/// `interface nsISecurityConsoleMessage : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISecurityConsoleMessage {
    vtable: *const nsISecurityConsoleMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISecurityConsoleMessage.
unsafe impl XpCom for nsISecurityConsoleMessage {
    const IID: nsIID = nsID(0xfe9fc9b6, 0xdde2, 0x11e2,
        [0xa8, 0xf1, 0x0a, 0x32, 0x61, 0x88, 0x70, 0x9b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISecurityConsoleMessage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISecurityConsoleMessage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISecurityConsoleMessageCoerce {
    /// Cheaply cast a value of this type from a `nsISecurityConsoleMessage`.
    fn coerce_from(v: &nsISecurityConsoleMessage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISecurityConsoleMessageCoerce for nsISecurityConsoleMessage {
    #[inline]
    fn coerce_from(v: &nsISecurityConsoleMessage) -> &Self {
        v
    }
}

impl nsISecurityConsoleMessage {
    /// Cast this `nsISecurityConsoleMessage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISecurityConsoleMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISecurityConsoleMessage {
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
impl<T: nsISupportsCoerce> nsISecurityConsoleMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecurityConsoleMessage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISecurityConsoleMessage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISecurityConsoleMessageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AString tag; */
    pub GetTag: unsafe extern "system" fn (this: *const nsISecurityConsoleMessage, aTag: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString tag; */
    pub SetTag: unsafe extern "system" fn (this: *const nsISecurityConsoleMessage, aTag: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString category; */
    pub GetCategory: unsafe extern "system" fn (this: *const nsISecurityConsoleMessage, aCategory: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString category; */
    pub SetCategory: unsafe extern "system" fn (this: *const nsISecurityConsoleMessage, aCategory: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISecurityConsoleMessage {


    /// `attribute AString tag;`
    #[inline]
    pub unsafe fn GetTag(&self, aTag: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTag)(self, aTag)
    }



    /// `attribute AString tag;`
    #[inline]
    pub unsafe fn SetTag(&self, aTag: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetTag)(self, aTag)
    }



    /// `attribute AString category;`
    #[inline]
    pub unsafe fn GetCategory(&self, aCategory: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCategory)(self, aCategory)
    }



    /// `attribute AString category;`
    #[inline]
    pub unsafe fn SetCategory(&self, aCategory: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetCategory)(self, aCategory)
    }


}


