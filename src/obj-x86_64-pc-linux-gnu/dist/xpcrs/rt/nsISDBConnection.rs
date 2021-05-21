//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBConnection.idl
//


/// `interface nsISDBConnection : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISDBConnection {
    vtable: *const nsISDBConnectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISDBConnection.
unsafe impl XpCom for nsISDBConnection {
    const IID: nsIID = nsID(0xea420fdd, 0x548f, 0x44f9,
        [0x92, 0x86, 0x59, 0xaa, 0xd6, 0xa4, 0x0f, 0x01]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISDBConnection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISDBConnection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISDBConnectionCoerce {
    /// Cheaply cast a value of this type from a `nsISDBConnection`.
    fn coerce_from(v: &nsISDBConnection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISDBConnectionCoerce for nsISDBConnection {
    #[inline]
    fn coerce_from(v: &nsISDBConnection) -> &Self {
        v
    }
}

impl nsISDBConnection {
    /// Cast this `nsISDBConnection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISDBConnectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISDBConnection {
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
impl<T: nsISupportsCoerce> nsISDBConnectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISDBConnection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISDBConnection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISDBConnectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void init (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType); */
    pub Init: unsafe extern "system" fn (this: *const nsISDBConnection, aPrincipal: *const nsIPrincipal, aPersistenceType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] nsISDBRequest open (in AString aName); */
    pub Open: unsafe extern "system" fn (this: *const nsISDBConnection, aName: *const ::nsstring::nsAString, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult,

    /* [must_use] nsISDBRequest seek (in unsigned long long offset); */
    pub Seek: unsafe extern "system" fn (this: *const nsISDBConnection, offset: u64, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult,

    /* [must_use] nsISDBRequest read (in unsigned long long size); */
    pub Read: unsafe extern "system" fn (this: *const nsISDBConnection, size: u64, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult,

    /* [implicit_jscontext,must_use] nsISDBRequest write (in jsval value); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Write: *const ::libc::c_void,

    /* [must_use] nsISDBRequest close (); */
    pub Close: unsafe extern "system" fn (this: *const nsISDBConnection, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult,

    /* attribute nsISDBCloseCallback closeCallback; */
    pub GetCloseCallback: unsafe extern "system" fn (this: *const nsISDBConnection, aCloseCallback: *mut*const nsISDBCloseCallback) -> ::nserror::nsresult,

    /* attribute nsISDBCloseCallback closeCallback; */
    pub SetCloseCallback: unsafe extern "system" fn (this: *const nsISDBConnection, aCloseCallback: *const nsISDBCloseCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISDBConnection {


    /// `[must_use] void init (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType);`
    #[inline]
    pub unsafe fn Init(&self, aPrincipal: *const nsIPrincipal, aPersistenceType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aPrincipal, aPersistenceType)
    }



    /// `[must_use] nsISDBRequest open (in AString aName);`
    #[inline]
    pub unsafe fn Open(&self, aName: *const ::nsstring::nsAString, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult {
        ((*self.vtable).Open)(self, aName, _retval)
    }



    /// `[must_use] nsISDBRequest seek (in unsigned long long offset);`
    #[inline]
    pub unsafe fn Seek(&self, offset: u64, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult {
        ((*self.vtable).Seek)(self, offset, _retval)
    }



    /// `[must_use] nsISDBRequest read (in unsigned long long size);`
    #[inline]
    pub unsafe fn Read(&self, size: u64, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult {
        ((*self.vtable).Read)(self, size, _retval)
    }



    /// `[implicit_jscontext,must_use] nsISDBRequest write (in jsval value);`
    const _Write: () = ();


    /// `[must_use] nsISDBRequest close ();`
    #[inline]
    pub unsafe fn Close(&self, _retval: *mut*const nsISDBRequest) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, _retval)
    }



    /// `attribute nsISDBCloseCallback closeCallback;`
    #[inline]
    pub unsafe fn GetCloseCallback(&self, aCloseCallback: *mut*const nsISDBCloseCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetCloseCallback)(self, aCloseCallback)
    }



    /// `attribute nsISDBCloseCallback closeCallback;`
    #[inline]
    pub unsafe fn SetCloseCallback(&self, aCloseCallback: *const nsISDBCloseCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetCloseCallback)(self, aCloseCallback)
    }


}


