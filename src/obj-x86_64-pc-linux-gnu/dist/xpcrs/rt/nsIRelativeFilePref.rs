//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIRelativeFilePref.idl
//


/// `interface nsIRelativeFilePref : nsISupports`
///

/// ```text
/// /**
///  * The nsIRelativeFilePref interface is a wrapper for an nsIFile and
///  * and a directory service key. When used as a pref value, it stores a
///  * relative path to the file from the location pointed to by the directory
///  * service key. The path has the same syntax across all platforms.
///  *
///  * @see nsIPrefBranch::getComplexValue
///  * @see nsIPrefBranch::setComplexValue
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRelativeFilePref {
    vtable: *const nsIRelativeFilePrefVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRelativeFilePref.
unsafe impl XpCom for nsIRelativeFilePref {
    const IID: nsIID = nsID(0x2f977d4e, 0x5485, 0x11d4,
        [0x87, 0xe2, 0x00, 0x10, 0xa4, 0xe7, 0x5e, 0xf2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRelativeFilePref {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRelativeFilePref.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRelativeFilePrefCoerce {
    /// Cheaply cast a value of this type from a `nsIRelativeFilePref`.
    fn coerce_from(v: &nsIRelativeFilePref) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRelativeFilePrefCoerce for nsIRelativeFilePref {
    #[inline]
    fn coerce_from(v: &nsIRelativeFilePref) -> &Self {
        v
    }
}

impl nsIRelativeFilePref {
    /// Cast this `nsIRelativeFilePref` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRelativeFilePrefCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRelativeFilePref {
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
impl<T: nsISupportsCoerce> nsIRelativeFilePrefCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRelativeFilePref) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRelativeFilePref
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRelativeFilePrefVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIFile file; */
    pub GetFile: unsafe extern "system" fn (this: *const nsIRelativeFilePref, aFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute nsIFile file; */
    pub SetFile: unsafe extern "system" fn (this: *const nsIRelativeFilePref, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* attribute ACString relativeToKey; */
    pub GetRelativeToKey: unsafe extern "system" fn (this: *const nsIRelativeFilePref, aRelativeToKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString relativeToKey; */
    pub SetRelativeToKey: unsafe extern "system" fn (this: *const nsIRelativeFilePref, aRelativeToKey: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRelativeFilePref {

    /// ```text
    /// /**
    ///    * file
    ///    *
    ///    * The file whose location is stored or retrieved.
    ///    */
    /// ```
    ///

    /// `attribute nsIFile file;`
    #[inline]
    pub unsafe fn GetFile(&self, aFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///    * file
    ///    *
    ///    * The file whose location is stored or retrieved.
    ///    */
    /// ```
    ///

    /// `attribute nsIFile file;`
    #[inline]
    pub unsafe fn SetFile(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SetFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///    * relativeToKey
    ///    *
    ///    * A directory service key for the directory
    ///    * from which the file path is relative.
    ///    */
    /// ```
    ///

    /// `attribute ACString relativeToKey;`
    #[inline]
    pub unsafe fn GetRelativeToKey(&self, aRelativeToKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRelativeToKey)(self, aRelativeToKey)
    }


    /// ```text
    /// /**
    ///    * relativeToKey
    ///    *
    ///    * A directory service key for the directory
    ///    * from which the file path is relative.
    ///    */
    /// ```
    ///

    /// `attribute ACString relativeToKey;`
    #[inline]
    pub unsafe fn SetRelativeToKey(&self, aRelativeToKey: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetRelativeToKey)(self, aRelativeToKey)
    }


}


