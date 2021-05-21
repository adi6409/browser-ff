//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/gfx/thebes/nsIFontLoadCompleteCallback.idl
//


/// `interface nsIFontLoadCompleteCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFontLoadCompleteCallback {
    vtable: *const nsIFontLoadCompleteCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFontLoadCompleteCallback.
unsafe impl XpCom for nsIFontLoadCompleteCallback {
    const IID: nsIID = nsID(0x302dbf09, 0x079b, 0x4648,
        [0x8a, 0x06, 0xa0, 0x48, 0x6c, 0x17, 0x49, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFontLoadCompleteCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFontLoadCompleteCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFontLoadCompleteCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIFontLoadCompleteCallback`.
    fn coerce_from(v: &nsIFontLoadCompleteCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFontLoadCompleteCallbackCoerce for nsIFontLoadCompleteCallback {
    #[inline]
    fn coerce_from(v: &nsIFontLoadCompleteCallback) -> &Self {
        v
    }
}

impl nsIFontLoadCompleteCallback {
    /// Cast this `nsIFontLoadCompleteCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFontLoadCompleteCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFontLoadCompleteCallback {
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
impl<T: nsISupportsCoerce> nsIFontLoadCompleteCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFontLoadCompleteCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFontLoadCompleteCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFontLoadCompleteCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void fontLoadComplete (); */
    pub FontLoadComplete: unsafe extern "system" fn (this: *const nsIFontLoadCompleteCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFontLoadCompleteCallback {


    /// `void fontLoadComplete ();`
    #[inline]
    pub unsafe fn FontLoadComplete(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FontLoadComplete)(self, )
    }


}


