//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPK11TokenDB.idl
//


/// `interface nsIPK11TokenDB : nsISupports`
///

/// ```text
/// /**
///  * The PK11 Token Database provides access to the PK11 modules
///  * that are installed, and the tokens that are available.
///  * Interfaces: nsIPK11TokenDB
///  * Threading: ??
///  */
/// /**
///  * nsIPK11TokenDB - Manages PK11 Tokens
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPK11TokenDB {
    vtable: *const nsIPK11TokenDBVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPK11TokenDB.
unsafe impl XpCom for nsIPK11TokenDB {
    const IID: nsIID = nsID(0x4ee28c82, 0x1dd2, 0x11b2,
        [0xaa, 0xbf, 0xbb, 0x40, 0x17, 0xab, 0xe3, 0x95]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPK11TokenDB {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPK11TokenDB.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPK11TokenDBCoerce {
    /// Cheaply cast a value of this type from a `nsIPK11TokenDB`.
    fn coerce_from(v: &nsIPK11TokenDB) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPK11TokenDBCoerce for nsIPK11TokenDB {
    #[inline]
    fn coerce_from(v: &nsIPK11TokenDB) -> &Self {
        v
    }
}

impl nsIPK11TokenDB {
    /// Cast this `nsIPK11TokenDB` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPK11TokenDBCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPK11TokenDB {
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
impl<T: nsISupportsCoerce> nsIPK11TokenDBCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPK11TokenDB) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPK11TokenDB
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPK11TokenDBVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIPK11Token getInternalKeyToken (); */
    pub GetInternalKeyToken: unsafe extern "system" fn (this: *const nsIPK11TokenDB, _retval: *mut*const nsIPK11Token) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPK11TokenDB {


    /// `nsIPK11Token getInternalKeyToken ();`
    #[inline]
    pub unsafe fn GetInternalKeyToken(&self, _retval: *mut*const nsIPK11Token) -> ::nserror::nsresult {
        ((*self.vtable).GetInternalKeyToken)(self, _retval)
    }


}


