//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIGtkTaskbarProgress.idl
//


/// `interface nsIGtkTaskbarProgress : nsITaskbarProgress`
///

/// ```text
/// /**
///  * Allow the TaskbarProgress instance to set a new target window.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGtkTaskbarProgress {
    vtable: *const nsIGtkTaskbarProgressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGtkTaskbarProgress.
unsafe impl XpCom for nsIGtkTaskbarProgress {
    const IID: nsIID = nsID(0x39f6fc5a, 0x2386, 0x4bc6,
        [0x94, 0x1c, 0xd7, 0x47, 0x92, 0x53, 0xbc, 0x3f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGtkTaskbarProgress {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGtkTaskbarProgress.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGtkTaskbarProgressCoerce {
    /// Cheaply cast a value of this type from a `nsIGtkTaskbarProgress`.
    fn coerce_from(v: &nsIGtkTaskbarProgress) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGtkTaskbarProgressCoerce for nsIGtkTaskbarProgress {
    #[inline]
    fn coerce_from(v: &nsIGtkTaskbarProgress) -> &Self {
        v
    }
}

impl nsIGtkTaskbarProgress {
    /// Cast this `nsIGtkTaskbarProgress` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGtkTaskbarProgressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGtkTaskbarProgress {
    type Target = nsITaskbarProgress;
    #[inline]
    fn deref(&self) -> &nsITaskbarProgress {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsITaskbarProgressCoerce> nsIGtkTaskbarProgressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGtkTaskbarProgress) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGtkTaskbarProgress
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGtkTaskbarProgressVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsITaskbarProgressVTable,

    /* void setPrimaryWindow (in mozIDOMWindowProxy aWindow); */
    pub SetPrimaryWindow: unsafe extern "system" fn (this: *const nsIGtkTaskbarProgress, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGtkTaskbarProgress {

    /// ```text
    /// /**
    ///    * Sets the window that is considered primary for purposes of
    ///    * setting the XApp progress property.
    ///    */
    /// ```
    ///

    /// `void setPrimaryWindow (in mozIDOMWindowProxy aWindow);`
    #[inline]
    pub unsafe fn SetPrimaryWindow(&self, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).SetPrimaryWindow)(self, aWindow)
    }


}


