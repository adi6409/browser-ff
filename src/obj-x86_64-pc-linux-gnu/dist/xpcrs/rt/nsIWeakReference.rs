//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIWeakReference.idl
//


/// `interface nsIWeakReference : nsISupports`
///

/// ```text
/// /**
///  * An instance of |nsIWeakReference| is a proxy object that cooperates with
///  * its referent to give clients a non-owning, non-dangling reference.  Clients
///  * own the proxy, and should generally manage it with an |nsCOMPtr| (see the
    ///  * type |nsWeakPtr| for a |typedef| name that stands out) as they would any
///  * other XPCOM object.  The |QueryReferent| member function provides a
///  * (hopefully short-lived) owning reference on demand, through which clients
///  * can get useful access to the referent, while it still exists.
///  *
///  * @version 1.0
///  * @see nsISupportsWeakReference
///  * @see nsWeakReference
///  * @see nsWeakPtr
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWeakReference {
    vtable: *const nsIWeakReferenceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWeakReference.
unsafe impl XpCom for nsIWeakReference {
    const IID: nsIID = nsID(0x9188bc85, 0xf92e, 0x11d2,
        [0x81, 0xef, 0x00, 0x60, 0x08, 0x3a, 0x0b, 0xcf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWeakReference {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWeakReference.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWeakReferenceCoerce {
    /// Cheaply cast a value of this type from a `nsIWeakReference`.
    fn coerce_from(v: &nsIWeakReference) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWeakReferenceCoerce for nsIWeakReference {
    #[inline]
    fn coerce_from(v: &nsIWeakReference) -> &Self {
        v
    }
}

impl nsIWeakReference {
    /// Cast this `nsIWeakReference` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWeakReferenceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWeakReference {
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
impl<T: nsISupportsCoerce> nsIWeakReferenceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWeakReference) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWeakReference
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWeakReferenceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [binaryname(QueryReferentFromScript)] void QueryReferent (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub QueryReferentFromScript: unsafe extern "system" fn (this: *const nsIWeakReference, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWeakReference {

    /// ```text
    /// /**
    ///      * |QueryReferent| queries the referent, if it exists, and like |QueryInterface|, produces
    ///      * an owning reference to the desired interface.  It is designed to look and act exactly
    ///      * like (a proxied) |QueryInterface|.  Don't hold on to the produced interface permanently;
    ///      * that would defeat the purpose of using a non-owning |nsIWeakReference| in the first place.
    ///      */
    /// ```
    ///

    /// `[binaryname(QueryReferentFromScript)] void QueryReferent (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn QueryReferentFromScript(&self, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).QueryReferentFromScript)(self, uuid, result)
    }


}


/// `interface nsISupportsWeakReference : nsISupports`
///

/// ```text
/// /**
///  * |nsISupportsWeakReference| is a factory interface which produces appropriate
///  * instances of |nsIWeakReference|.  Weak references in this scheme can only be
///  * produced for objects that implement this interface.
///  *
///  * @version 1.0
///  * @see nsIWeakReference
///  * @see nsSupportsWeakReference
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsWeakReference {
    vtable: *const nsISupportsWeakReferenceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsWeakReference.
unsafe impl XpCom for nsISupportsWeakReference {
    const IID: nsIID = nsID(0x9188bc86, 0xf92e, 0x11d2,
        [0x81, 0xef, 0x00, 0x60, 0x08, 0x3a, 0x0b, 0xcf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsWeakReference {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsWeakReference.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsWeakReferenceCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsWeakReference`.
    fn coerce_from(v: &nsISupportsWeakReference) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsWeakReferenceCoerce for nsISupportsWeakReference {
    #[inline]
    fn coerce_from(v: &nsISupportsWeakReference) -> &Self {
        v
    }
}

impl nsISupportsWeakReference {
    /// Cast this `nsISupportsWeakReference` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsWeakReferenceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsWeakReference {
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
impl<T: nsISupportsCoerce> nsISupportsWeakReferenceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsWeakReference) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsWeakReference
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsWeakReferenceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIWeakReference GetWeakReference (); */
    pub GetWeakReference: unsafe extern "system" fn (this: *const nsISupportsWeakReference, _retval: *mut *const nsIWeakReference) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsWeakReference {

    /// ```text
    /// /**
    ///      * |GetWeakReference| produces an appropriate instance of |nsIWeakReference|.
    ///      * As with all good XPCOM `getters', you own the resulting interface and should
    ///      * manage it with an |nsCOMPtr|.
    ///      *
    ///      * @see nsIWeakReference
    ///      * @see nsWeakPtr
    ///      * @see nsCOMPtr
    ///      */
    /// ```
    ///

    /// `nsIWeakReference GetWeakReference ();`
    #[inline]
    pub unsafe fn GetWeakReference(&self, _retval: *mut *const nsIWeakReference) -> ::nserror::nsresult {
        ((*self.vtable).GetWeakReference)(self, _retval)
    }


}


