//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFileURL.idl
//


/// `interface nsIFileURL : nsIURL`
///

/// ```text
/// /**
///  * nsIFileURL provides access to the underlying nsIFile object corresponding to
///  * an URL.  The URL scheme need not be file:, since other local protocols may
///  * map URLs to files (e.g., resource:).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileURL {
    vtable: *const nsIFileURLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileURL.
unsafe impl XpCom for nsIFileURL {
    const IID: nsIID = nsID(0xe91ac988, 0x27c2, 0x448b,
        [0xb1, 0xa1, 0x38, 0x22, 0xe1, 0xef, 0x19, 0x87]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileURL {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileURL.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileURLCoerce {
    /// Cheaply cast a value of this type from a `nsIFileURL`.
    fn coerce_from(v: &nsIFileURL) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileURLCoerce for nsIFileURL {
    #[inline]
    fn coerce_from(v: &nsIFileURL) -> &Self {
        v
    }
}

impl nsIFileURL {
    /// Cast this `nsIFileURL` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileURLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileURL {
    type Target = nsIURL;
    #[inline]
    fn deref(&self) -> &nsIURL {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIURLCoerce> nsIFileURLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileURL) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileURL
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileURLVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIURLVTable,

    /* readonly attribute nsIFile file; */
    pub GetFile: unsafe extern "system" fn (this: *const nsIFileURL, aFile: *mut*const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileURL {

    /// ```text
    /// /**
    ///      * Get the nsIFile corresponding to this URL.
    ///      *
    ///      *  - Returns a reference to an immutable object.  Callers must clone
    ///      *    before attempting to modify the returned nsIFile object.  NOTE: this
    ///      *    constraint might not be enforced at runtime, so beware!!
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile file;`
    #[inline]
    pub unsafe fn GetFile(&self, aFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFile)(self, aFile)
    }


}


/// `interface nsIFileURLMutator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileURLMutator {
    vtable: *const nsIFileURLMutatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileURLMutator.
unsafe impl XpCom for nsIFileURLMutator {
    const IID: nsIID = nsID(0xa588b6f2, 0xd2b9, 0x4024,
        [0x84, 0xc7, 0xbe, 0x33, 0x68, 0x54, 0x6b, 0x57]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileURLMutator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileURLMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileURLMutatorCoerce {
    /// Cheaply cast a value of this type from a `nsIFileURLMutator`.
    fn coerce_from(v: &nsIFileURLMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileURLMutatorCoerce for nsIFileURLMutator {
    #[inline]
    fn coerce_from(v: &nsIFileURLMutator) -> &Self {
        v
    }
}

impl nsIFileURLMutator {
    /// Cast this `nsIFileURLMutator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileURLMutatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileURLMutator {
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
impl<T: nsISupportsCoerce> nsIFileURLMutatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileURLMutator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileURLMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileURLMutatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use,noscript] void markFileURL (); */
    pub MarkFileURL: unsafe extern "system" fn (this: *const nsIFileURLMutator) -> ::nserror::nsresult,

    /* [must_use,noscript] void setFile (in nsIFile aFile); */
    pub SetFile: unsafe extern "system" fn (this: *const nsIFileURLMutator, aFile: *const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileURLMutator {


    /// `[must_use,noscript] void markFileURL ();`
    #[inline]
    pub unsafe fn MarkFileURL(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MarkFileURL)(self, )
    }



    /// `[must_use,noscript] void setFile (in nsIFile aFile);`
    #[inline]
    pub unsafe fn SetFile(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SetFile)(self, aFile)
    }


}


