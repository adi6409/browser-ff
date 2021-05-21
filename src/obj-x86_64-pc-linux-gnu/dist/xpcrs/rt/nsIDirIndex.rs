//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIDirIndex.idl
//


/// `interface nsIDirIndex : nsISupports`
///

/// ```text
/// /** A class holding information about a directory index.
///  * These have no reference back to their original source -
///  * changing these attributes won't affect the directory
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirIndex {
    vtable: *const nsIDirIndexVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirIndex.
unsafe impl XpCom for nsIDirIndex {
    const IID: nsIID = nsID(0x23bbabd0, 0x1dd2, 0x11b2,
        [0x86, 0xb7, 0xaa, 0xd6, 0x8a, 0xe7, 0xd7, 0xe0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirIndex {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirIndex.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirIndexCoerce {
    /// Cheaply cast a value of this type from a `nsIDirIndex`.
    fn coerce_from(v: &nsIDirIndex) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirIndexCoerce for nsIDirIndex {
    #[inline]
    fn coerce_from(v: &nsIDirIndex) -> &Self {
        v
    }
}

impl nsIDirIndex {
    /// Cast this `nsIDirIndex` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirIndexCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirIndex {
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
impl<T: nsISupportsCoerce> nsIDirIndexCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirIndex) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirIndex
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirIndexVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute unsigned long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIDirIndex, aType: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long type; */
    pub SetType: unsafe extern "system" fn (this: *const nsIDirIndex, aType: u32) -> ::nserror::nsresult,

    /* attribute ACString contentType; */
    pub GetContentType: unsafe extern "system" fn (this: *const nsIDirIndex, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString contentType; */
    pub SetContentType: unsafe extern "system" fn (this: *const nsIDirIndex, aContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString location; */
    pub GetLocation: unsafe extern "system" fn (this: *const nsIDirIndex, aLocation: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString location; */
    pub SetLocation: unsafe extern "system" fn (this: *const nsIDirIndex, aLocation: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AString description; */
    pub GetDescription: unsafe extern "system" fn (this: *const nsIDirIndex, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString description; */
    pub SetDescription: unsafe extern "system" fn (this: *const nsIDirIndex, aDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute long long size; */
    pub GetSize: unsafe extern "system" fn (this: *const nsIDirIndex, aSize: *mut i64) -> ::nserror::nsresult,

    /* attribute long long size; */
    pub SetSize: unsafe extern "system" fn (this: *const nsIDirIndex, aSize: i64) -> ::nserror::nsresult,

    /* attribute PRTime lastModified; */
    pub GetLastModified: unsafe extern "system" fn (this: *const nsIDirIndex, aLastModified: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime lastModified; */
    pub SetLastModified: unsafe extern "system" fn (this: *const nsIDirIndex, aLastModified: PRTime) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirIndex {
    /// ```text
    /// /**
    ///      * Entry's type is unknown
    ///      */
    /// ```
    ///

    pub const TYPE_UNKNOWN: i64 = 0;

    /// ```text
    /// /**
    ///      * Entry is a directory
    ///      */
    /// ```
    ///

    pub const TYPE_DIRECTORY: i64 = 1;

    /// ```text
    /// /**
    ///      * Entry is a file
    ///      */
    /// ```
    ///

    pub const TYPE_FILE: i64 = 2;

    /// ```text
    /// /**
    ///      * Entry is a symlink
    ///      */
    /// ```
    ///

    pub const TYPE_SYMLINK: i64 = 3;

    /// ```text
    /// /**
    ///      * The type of the entry - one of the constants above
    ///      */
    /// ```
    ///

    /// `attribute unsigned long type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///      * The type of the entry - one of the constants above
    ///      */
    /// ```
    ///

    /// `attribute unsigned long type;`
    #[inline]
    pub unsafe fn SetType(&self, aType: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetType)(self, aType)
    }


    /// ```text
    /// /**
    ///      * The content type - may be null if it is unknown.
    ///      * Unspecified for directories
    ///      */
    /// ```
    ///

    /// `attribute ACString contentType;`
    #[inline]
    pub unsafe fn GetContentType(&self, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentType)(self, aContentType)
    }


    /// ```text
    /// /**
    ///      * The content type - may be null if it is unknown.
    ///      * Unspecified for directories
    ///      */
    /// ```
    ///

    /// `attribute ACString contentType;`
    #[inline]
    pub unsafe fn SetContentType(&self, aContentType: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetContentType)(self, aContentType)
    }


    /// ```text
    /// /**
    ///      * The fully qualified filename, expressed as a uri
    ///      *
    ///      * This is encoded with the encoding specified in
    ///      * the nsIDirIndexParser, and is also escaped.
    ///      */
    /// ```
    ///

    /// `attribute ACString location;`
    #[inline]
    pub unsafe fn GetLocation(&self, aLocation: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLocation)(self, aLocation)
    }


    /// ```text
    /// /**
    ///      * The fully qualified filename, expressed as a uri
    ///      *
    ///      * This is encoded with the encoding specified in
    ///      * the nsIDirIndexParser, and is also escaped.
    ///      */
    /// ```
    ///

    /// `attribute ACString location;`
    #[inline]
    pub unsafe fn SetLocation(&self, aLocation: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetLocation)(self, aLocation)
    }


    /// ```text
    /// /**
    ///      * A description for the filename, which should be
    ///      * displayed by a viewer
    ///      */
    /// ```
    ///

    /// `attribute AString description;`
    #[inline]
    pub unsafe fn GetDescription(&self, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescription)(self, aDescription)
    }


    /// ```text
    /// /**
    ///      * A description for the filename, which should be
    ///      * displayed by a viewer
    ///      */
    /// ```
    ///

    /// `attribute AString description;`
    #[inline]
    pub unsafe fn SetDescription(&self, aDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDescription)(self, aDescription)
    }


    /// ```text
    /// /**
    ///      * File size, with -1 meaning "unknown"
    ///      */
    /// ```
    ///

    /// `attribute long long size;`
    #[inline]
    pub unsafe fn GetSize(&self, aSize: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetSize)(self, aSize)
    }


    /// ```text
    /// /**
    ///      * File size, with -1 meaning "unknown"
    ///      */
    /// ```
    ///

    /// `attribute long long size;`
    #[inline]
    pub unsafe fn SetSize(&self, aSize: i64) -> ::nserror::nsresult {
        ((*self.vtable).SetSize)(self, aSize)
    }


    /// ```text
    /// /**
    ///      * Last-modified time in seconds-since-epoch.
    ///      * -1 means unknown - this is valid, because there were no
    ///      * ftp servers in 1969
    ///      */
    /// ```
    ///

    /// `attribute PRTime lastModified;`
    #[inline]
    pub unsafe fn GetLastModified(&self, aLastModified: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModified)(self, aLastModified)
    }


    /// ```text
    /// /**
    ///      * Last-modified time in seconds-since-epoch.
    ///      * -1 means unknown - this is valid, because there were no
    ///      * ftp servers in 1969
    ///      */
    /// ```
    ///

    /// `attribute PRTime lastModified;`
    #[inline]
    pub unsafe fn SetLastModified(&self, aLastModified: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).SetLastModified)(self, aLastModified)
    }


}


