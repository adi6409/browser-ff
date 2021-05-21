//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINestedURI.idl
//


/// `interface nsINestedURI : nsISupports`
///

/// ```text
/// /**
///  * nsINestedURI is an interface that must be implemented by any nsIURI
///  * implementation which has an "inner" URI that it actually gets data
///  * from.
///  *
///  * For example, if URIs for the scheme "sanitize" have the structure:
///  *
///  *   sanitize:http://example.com
///  *
///  * and opening a channel on such a sanitize: URI gets the data from
///  * http://example.com, sanitizes it, and returns it, then the sanitize: URI
///  * should implement nsINestedURI and return the http://example.com URI as its
///  * inner URI.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINestedURI {
    vtable: *const nsINestedURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINestedURI.
unsafe impl XpCom for nsINestedURI {
    const IID: nsIID = nsID(0x6de2c874, 0x796c, 0x46bf,
        [0xb5, 0x7f, 0x0d, 0x7b, 0xd7, 0xd6, 0xca, 0xb0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINestedURI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINestedURI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINestedURICoerce {
    /// Cheaply cast a value of this type from a `nsINestedURI`.
    fn coerce_from(v: &nsINestedURI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINestedURICoerce for nsINestedURI {
    #[inline]
    fn coerce_from(v: &nsINestedURI) -> &Self {
        v
    }
}

impl nsINestedURI {
    /// Cast this `nsINestedURI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINestedURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINestedURI {
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
impl<T: nsISupportsCoerce> nsINestedURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsINestedURI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINestedURI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINestedURIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIURI innerURI; */
    pub GetInnerURI: unsafe extern "system" fn (this: *const nsINestedURI, aInnerURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIURI innermostURI; */
    pub GetInnermostURI: unsafe extern "system" fn (this: *const nsINestedURI, aInnermostURI: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINestedURI {

    /// ```text
    /// /**
    ///    * The inner URI for this nested URI.  This must not return null if the
    ///    * getter succeeds; URIs that have no inner must not QI to this interface.
    ///    * Dynamically changing whether there is an inner URI is not allowed.
    ///    *
    ///    * Modifying the returned URI must not in any way modify the nested URI; this
    ///    * means the returned URI must be either immutable or a clone.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI innerURI;`
    #[inline]
    pub unsafe fn GetInnerURI(&self, aInnerURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetInnerURI)(self, aInnerURI)
    }


    /// ```text
    /// /**
    ///    * The innermost URI for this nested URI.  This must not return null if the
    ///    * getter succeeds.  This is equivalent to repeatedly calling innerURI while
    ///    * the returned URI QIs to nsINestedURI.
    ///    *
    ///    * Modifying the returned URI must not in any way modify the nested URI; this
    ///    * means the returned URI must be either immutable or a clone.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI innermostURI;`
    #[inline]
    pub unsafe fn GetInnermostURI(&self, aInnermostURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetInnermostURI)(self, aInnermostURI)
    }


}


/// `interface nsINestedURIMutator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINestedURIMutator {
    vtable: *const nsINestedURIMutatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINestedURIMutator.
unsafe impl XpCom for nsINestedURIMutator {
    const IID: nsIID = nsID(0xca3d6c03, 0x4eee, 0x4271,
        [0xa9, 0x7a, 0xd1, 0x6c, 0x0a, 0x0b, 0x2c, 0x5c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINestedURIMutator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINestedURIMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINestedURIMutatorCoerce {
    /// Cheaply cast a value of this type from a `nsINestedURIMutator`.
    fn coerce_from(v: &nsINestedURIMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINestedURIMutatorCoerce for nsINestedURIMutator {
    #[inline]
    fn coerce_from(v: &nsINestedURIMutator) -> &Self {
        v
    }
}

impl nsINestedURIMutator {
    /// Cast this `nsINestedURIMutator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINestedURIMutatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINestedURIMutator {
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
impl<T: nsISupportsCoerce> nsINestedURIMutatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINestedURIMutator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINestedURIMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINestedURIMutatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use,noscript] void init (in nsIURI innerURI); */
    pub Init: unsafe extern "system" fn (this: *const nsINestedURIMutator, innerURI: *const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINestedURIMutator {


    /// `[must_use,noscript] void init (in nsIURI innerURI);`
    #[inline]
    pub unsafe fn Init(&self, innerURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, innerURI)
    }


}


/// `interface nsINestedAboutURIMutator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINestedAboutURIMutator {
    vtable: *const nsINestedAboutURIMutatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINestedAboutURIMutator.
unsafe impl XpCom for nsINestedAboutURIMutator {
    const IID: nsIID = nsID(0xc6357a3b, 0xc2bb, 0x4b4b,
        [0x92, 0x78, 0x51, 0x33, 0x77, 0x39, 0x8a, 0x38]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINestedAboutURIMutator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINestedAboutURIMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINestedAboutURIMutatorCoerce {
    /// Cheaply cast a value of this type from a `nsINestedAboutURIMutator`.
    fn coerce_from(v: &nsINestedAboutURIMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINestedAboutURIMutatorCoerce for nsINestedAboutURIMutator {
    #[inline]
    fn coerce_from(v: &nsINestedAboutURIMutator) -> &Self {
        v
    }
}

impl nsINestedAboutURIMutator {
    /// Cast this `nsINestedAboutURIMutator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINestedAboutURIMutatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINestedAboutURIMutator {
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
impl<T: nsISupportsCoerce> nsINestedAboutURIMutatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINestedAboutURIMutator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINestedAboutURIMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINestedAboutURIMutatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use,noscript] void initWithBase (in nsIURI innerURI, in nsIURI baseURI); */
    pub InitWithBase: unsafe extern "system" fn (this: *const nsINestedAboutURIMutator, innerURI: *const nsIURI, baseURI: *const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINestedAboutURIMutator {


    /// `[must_use,noscript] void initWithBase (in nsIURI innerURI, in nsIURI baseURI);`
    #[inline]
    pub unsafe fn InitWithBase(&self, innerURI: *const nsIURI, baseURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).InitWithBase)(self, innerURI, baseURI)
    }


}


/// `interface nsIJSURIMutator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIJSURIMutator {
    vtable: *const nsIJSURIMutatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIJSURIMutator.
unsafe impl XpCom for nsIJSURIMutator {
    const IID: nsIID = nsID(0x3bd44535, 0x08ea, 0x478f,
        [0x99, 0xb9, 0x85, 0xfa, 0x10, 0x84, 0xe8, 0x20]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIJSURIMutator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIJSURIMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIJSURIMutatorCoerce {
    /// Cheaply cast a value of this type from a `nsIJSURIMutator`.
    fn coerce_from(v: &nsIJSURIMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIJSURIMutatorCoerce for nsIJSURIMutator {
    #[inline]
    fn coerce_from(v: &nsIJSURIMutator) -> &Self {
        v
    }
}

impl nsIJSURIMutator {
    /// Cast this `nsIJSURIMutator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIJSURIMutatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIJSURIMutator {
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
impl<T: nsISupportsCoerce> nsIJSURIMutatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSURIMutator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIJSURIMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIJSURIMutatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use,noscript] void setBase (in nsIURI aBaseURI); */
    pub SetBase: unsafe extern "system" fn (this: *const nsIJSURIMutator, aBaseURI: *const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIJSURIMutator {


    /// `[must_use,noscript] void setBase (in nsIURI aBaseURI);`
    #[inline]
    pub unsafe fn SetBase(&self, aBaseURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetBase)(self, aBaseURI)
    }


}


