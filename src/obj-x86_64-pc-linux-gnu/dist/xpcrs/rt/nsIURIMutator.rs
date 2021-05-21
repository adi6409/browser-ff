//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURIMutator.idl
//


/// `interface nsIURISetSpec : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURISetSpec {
    vtable: *const nsIURISetSpecVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURISetSpec.
unsafe impl XpCom for nsIURISetSpec {
    const IID: nsIID = nsID(0x1fc53257, 0x898b, 0x4c5e,
        [0xb6, 0x9c, 0x05, 0xbc, 0x84, 0xb4, 0xcd, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURISetSpec {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURISetSpec.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURISetSpecCoerce {
    /// Cheaply cast a value of this type from a `nsIURISetSpec`.
    fn coerce_from(v: &nsIURISetSpec) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURISetSpecCoerce for nsIURISetSpec {
    #[inline]
    fn coerce_from(v: &nsIURISetSpec) -> &Self {
        v
    }
}

impl nsIURISetSpec {
    /// Cast this `nsIURISetSpec` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURISetSpecCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURISetSpec {
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
impl<T: nsISupportsCoerce> nsIURISetSpecCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURISetSpec) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURISetSpec
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURISetSpecVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] nsIURIMutator setSpec (in AUTF8String aSpec); */
    pub SetSpec: unsafe extern "system" fn (this: *const nsIURISetSpec, aSpec: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURISetSpec {

    /// ```text
    /// /**
    ///    * This setter is different from all other setters because it may be used to
    ///    * initialize the object. We define it separately allowing mutator implementors
    ///    * to define it separately, while the rest of the setters may be simply
    ///    * forwarded to the mutable URI.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIURIMutator setSpec (in AUTF8String aSpec);`
    #[inline]
    pub unsafe fn SetSpec(&self, aSpec: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetSpec)(self, aSpec, _retval)
    }


}


/// `interface nsIURISetters : nsIURISetSpec`
///

/// ```text
/// /**
///  * These methods allow the mutator to change various parts of the URI.
///  * They return the same nsIURIMutator so that we may chain setter operations:
///  * Example:
///  * let newURI = uri.mutate()
///  *                 .setSpec("http://example.com")
///  *                 .setQuery("hello")
///  *                 .finalize();
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURISetters {
    vtable: *const nsIURISettersVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURISetters.
unsafe impl XpCom for nsIURISetters {
    const IID: nsIID = nsID(0x5403a6ec, 0x99d7, 0x405e,
        [0x8b, 0x45, 0x9f, 0x80, 0x5b, 0xbd, 0xfc, 0xef]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURISetters {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURISetters.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURISettersCoerce {
    /// Cheaply cast a value of this type from a `nsIURISetters`.
    fn coerce_from(v: &nsIURISetters) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURISettersCoerce for nsIURISetters {
    #[inline]
    fn coerce_from(v: &nsIURISetters) -> &Self {
        v
    }
}

impl nsIURISetters {
    /// Cast this `nsIURISetters` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURISettersCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURISetters {
    type Target = nsIURISetSpec;
    #[inline]
    fn deref(&self) -> &nsIURISetSpec {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIURISetSpecCoerce> nsIURISettersCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURISetters) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURISetters
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURISettersVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIURISetSpecVTable,

    /* [must_use] nsIURIMutator setScheme (in AUTF8String aScheme); */
    pub SetScheme: unsafe extern "system" fn (this: *const nsIURISetters, aScheme: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setUserPass (in AUTF8String aUserPass); */
    pub SetUserPass: unsafe extern "system" fn (this: *const nsIURISetters, aUserPass: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setUsername (in AUTF8String aUsername); */
    pub SetUsername: unsafe extern "system" fn (this: *const nsIURISetters, aUsername: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setPassword (in AUTF8String aPassword); */
    pub SetPassword: unsafe extern "system" fn (this: *const nsIURISetters, aPassword: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setHostPort (in AUTF8String aHostPort); */
    pub SetHostPort: unsafe extern "system" fn (this: *const nsIURISetters, aHostPort: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setHost (in AUTF8String aHost); */
    pub SetHost: unsafe extern "system" fn (this: *const nsIURISetters, aHost: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setPort (in long aPort); */
    pub SetPort: unsafe extern "system" fn (this: *const nsIURISetters, aPort: i32, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setPathQueryRef (in AUTF8String aPathQueryRef); */
    pub SetPathQueryRef: unsafe extern "system" fn (this: *const nsIURISetters, aPathQueryRef: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setRef (in AUTF8String aRef); */
    pub SetRef: unsafe extern "system" fn (this: *const nsIURISetters, aRef: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setFilePath (in AUTF8String aFilePath); */
    pub SetFilePath: unsafe extern "system" fn (this: *const nsIURISetters, aFilePath: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use] nsIURIMutator setQuery (in AUTF8String aQuery); */
    pub SetQuery: unsafe extern "system" fn (this: *const nsIURISetters, aQuery: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [must_use,noscript] nsIURIMutator setQueryWithEncoding (in AUTF8String query, in Encoding encoding); */
    /// Unable to generate binding because `native type const mozilla::Encoding unsupported`
    pub SetQueryWithEncoding: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURISetters {

    /// ```text
    /// /**
    ///    * Setting the scheme outside of a protocol handler implementation is highly
    ///    * discouraged since that will generally lead to incorrect results.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIURIMutator setScheme (in AUTF8String aScheme);`
    #[inline]
    pub unsafe fn SetScheme(&self, aScheme: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetScheme)(self, aScheme, _retval)
    }



    /// `[must_use] nsIURIMutator setUserPass (in AUTF8String aUserPass);`
    #[inline]
    pub unsafe fn SetUserPass(&self, aUserPass: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetUserPass)(self, aUserPass, _retval)
    }



    /// `[must_use] nsIURIMutator setUsername (in AUTF8String aUsername);`
    #[inline]
    pub unsafe fn SetUsername(&self, aUsername: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetUsername)(self, aUsername, _retval)
    }



    /// `[must_use] nsIURIMutator setPassword (in AUTF8String aPassword);`
    #[inline]
    pub unsafe fn SetPassword(&self, aPassword: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetPassword)(self, aPassword, _retval)
    }


    /// ```text
    /// /**
    ///    * If you setHostPort to a value that only has a host part, the port
    ///    * will not be reset. To reset the port set it to -1 beforehand.
    ///    * If setting the host succeeds, this method will return NS_OK, even if
    ///    * setting the port fails (error in parsing the port, or value out of range)
    ///    */
    /// ```
    ///

    /// `[must_use] nsIURIMutator setHostPort (in AUTF8String aHostPort);`
    #[inline]
    pub unsafe fn SetHostPort(&self, aHostPort: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetHostPort)(self, aHostPort, _retval)
    }



    /// `[must_use] nsIURIMutator setHost (in AUTF8String aHost);`
    #[inline]
    pub unsafe fn SetHost(&self, aHost: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetHost)(self, aHost, _retval)
    }



    /// `[must_use] nsIURIMutator setPort (in long aPort);`
    #[inline]
    pub unsafe fn SetPort(&self, aPort: i32, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetPort)(self, aPort, _retval)
    }



    /// `[must_use] nsIURIMutator setPathQueryRef (in AUTF8String aPathQueryRef);`
    #[inline]
    pub unsafe fn SetPathQueryRef(&self, aPathQueryRef: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetPathQueryRef)(self, aPathQueryRef, _retval)
    }



    /// `[must_use] nsIURIMutator setRef (in AUTF8String aRef);`
    #[inline]
    pub unsafe fn SetRef(&self, aRef: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetRef)(self, aRef, _retval)
    }



    /// `[must_use] nsIURIMutator setFilePath (in AUTF8String aFilePath);`
    #[inline]
    pub unsafe fn SetFilePath(&self, aFilePath: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetFilePath)(self, aFilePath, _retval)
    }



    /// `[must_use] nsIURIMutator setQuery (in AUTF8String aQuery);`
    #[inline]
    pub unsafe fn SetQuery(&self, aQuery: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).SetQuery)(self, aQuery, _retval)
    }



    /// `[must_use,noscript] nsIURIMutator setQueryWithEncoding (in AUTF8String query, in Encoding encoding);`
    const _SetQueryWithEncoding: () = ();

}


/// `interface nsIURIMutator : nsIURISetters`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURIMutator {
    vtable: *const nsIURIMutatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURIMutator.
unsafe impl XpCom for nsIURIMutator {
    const IID: nsIID = nsID(0x4d1f3103, 0x1c44, 0x4dcd,
        [0xb7, 0x17, 0x5d, 0x22, 0xa6, 0x97, 0xa7, 0xd9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURIMutator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURIMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURIMutatorCoerce {
    /// Cheaply cast a value of this type from a `nsIURIMutator`.
    fn coerce_from(v: &nsIURIMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURIMutatorCoerce for nsIURIMutator {
    #[inline]
    fn coerce_from(v: &nsIURIMutator) -> &Self {
        v
    }
}

impl nsIURIMutator {
    /// Cast this `nsIURIMutator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURIMutatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURIMutator {
    type Target = nsIURISetters;
    #[inline]
    fn deref(&self) -> &nsIURISetters {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIURISettersCoerce> nsIURIMutatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIMutator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURIMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURIMutatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIURISettersVTable,

    /* [must_use,noscript,notxpcom] nsresult deserialize (in const_URIParams_ref aParams); */
    /// Unable to generate binding because `native type const mozilla::ipc::URIParams unsupported`
    pub Deserialize: *const ::libc::c_void,

    /* [must_use] nsIURI finalize (); */
    pub Finalize: unsafe extern "system" fn (this: *const nsIURIMutator, _retval: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURIMutator {

    /// ```text
    /// /**
    ///    * Initalizes the URI by reading IPC URIParams.
    ///    * See nsIURI.
    ///    */
    /// ```
    ///

    /// `[must_use,noscript,notxpcom] nsresult deserialize (in const_URIParams_ref aParams);`
    const _Deserialize: () = ();

    /// ```text
    /// /**
    ///    * Finishes changing or constructing the URI and returns an immutable URI.
    ///    */
    /// ```
    ///

    /// `[must_use] nsIURI finalize ();`
    #[inline]
    pub unsafe fn Finalize(&self, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).Finalize)(self, _retval)
    }


}


