//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsIURLDecorationAnnotationsService.idl
//


/// `interface nsIURLDecorationAnnotationsService : nsISupports`
///

/// ```text
/// /**
///  * A service that monitors updates to the anti-tracking URL decoration
///  * annotations from remote settings.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURLDecorationAnnotationsService {
    vtable: *const nsIURLDecorationAnnotationsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURLDecorationAnnotationsService.
unsafe impl XpCom for nsIURLDecorationAnnotationsService {
    const IID: nsIID = nsID(0x937d0c66, 0x6821, 0x4e3f,
        [0x9e, 0x04, 0x50, 0xdb, 0xc2, 0xb2, 0xb4, 0x76]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURLDecorationAnnotationsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURLDecorationAnnotationsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURLDecorationAnnotationsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIURLDecorationAnnotationsService`.
    fn coerce_from(v: &nsIURLDecorationAnnotationsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURLDecorationAnnotationsServiceCoerce for nsIURLDecorationAnnotationsService {
    #[inline]
    fn coerce_from(v: &nsIURLDecorationAnnotationsService) -> &Self {
        v
    }
}

impl nsIURLDecorationAnnotationsService {
    /// Cast this `nsIURLDecorationAnnotationsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURLDecorationAnnotationsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURLDecorationAnnotationsService {
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
impl<T: nsISupportsCoerce> nsIURLDecorationAnnotationsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURLDecorationAnnotationsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURLDecorationAnnotationsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURLDecorationAnnotationsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Promise ensureUpdated (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub EnsureUpdated: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURLDecorationAnnotationsService {

    /// ```text
    /// /**
    ///    * Ensures that the list is updated and resolves the returned promise when
    ///    * the update is finished.
    ///    *
    ///    * The new list will be written to a space-separated list of tokens inside
    ///    * the following string preference:
    ///    *   privacy.restrict3rdpartystorage.url_decorations
    ///    *
    ///    * This preference will be kept up to date with future list updates from
    ///    * the remote settings server.  This preference cannot be modified by any
    ///    * external component and is managed by this service.
    ///    */
    /// ```
    ///

    /// `Promise ensureUpdated ();`
    const _EnsureUpdated: () = ();

}


