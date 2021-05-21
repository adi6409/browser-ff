//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/nsIIconURI.idl
//


/// `interface nsIMozIconURI : nsIURI`
///

/// ```text
/// /**
///    * nsIIconURI
///    *
///    * This interface derives from nsIURI, to provide additional information
///    * about moz-icon URIs.
///    *
///    * What *is* a moz-icon URI you ask?  Well, it has the following syntax:
///    *
///    * moz-icon:[<valid-url> | //<file-with-extension> | //stock/<stock-icon>]?
///    *            ['?'[<parameter-value-pairs>]]
///    *
///    * <valid-url> is a valid URL spec.
///    *
///    * <file-with-extension> is any filename with an extension, e.g. "dummy.html".
///    * If the file you want an icon for isn't known to exist, you can use this
///    * instead of a URL and just place a dummy file name with the extension or
///    * content type you want.
///    *
///    * <stock-icon> is the name of a platform-dependant stock icon.
///    *
///    * Legal parameter value pairs are listed below:
///    *
///    *   Parameter:   size
///    *   Values:      [<integer> | button | toolbar | toolbarsmall | menu |
    ///    *                 dialog]
///    *   Description: If integer, this is the desired size in square pixels of
///    *                the icon
///    *                Else, use the OS default for the specified keyword context.
///    *
///    *   Parameter:   state
///    *   Values:      [normal | disabled]
///    *   Description: The state of the icon.
///    *
///    *   Parameter:   contentType
///    *   Values:      <mime-type>
///    *   Description: The mime type we want an icon for. This is ignored by
///    *                stock images.
///    */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMozIconURI {
    vtable: *const nsIMozIconURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMozIconURI.
unsafe impl XpCom for nsIMozIconURI {
    const IID: nsIID = nsID(0xf8fe5ef2, 0x5f2b, 0x43f3,
        [0x85, 0x7d, 0x5b, 0x64, 0xd1, 0x92, 0xc4, 0x27]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMozIconURI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMozIconURI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMozIconURICoerce {
    /// Cheaply cast a value of this type from a `nsIMozIconURI`.
    fn coerce_from(v: &nsIMozIconURI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMozIconURICoerce for nsIMozIconURI {
    #[inline]
    fn coerce_from(v: &nsIMozIconURI) -> &Self {
        v
    }
}

impl nsIMozIconURI {
    /// Cast this `nsIMozIconURI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMozIconURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMozIconURI {
    type Target = nsIURI;
    #[inline]
    fn deref(&self) -> &nsIURI {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIURICoerce> nsIMozIconURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMozIconURI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMozIconURI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMozIconURIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIURIVTable,

    /* readonly attribute nsIURL iconURL; */
    pub GetIconURL: unsafe extern "system" fn (this: *const nsIMozIconURI, aIconURL: *mut *const nsIURL) -> ::nserror::nsresult,

    /* readonly attribute unsigned long imageSize; */
    pub GetImageSize: unsafe extern "system" fn (this: *const nsIMozIconURI, aImageSize: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute ACString stockIcon; */
    pub GetStockIcon: unsafe extern "system" fn (this: *const nsIMozIconURI, aStockIcon: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString iconSize; */
    pub GetIconSize: unsafe extern "system" fn (this: *const nsIMozIconURI, aIconSize: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString iconState; */
    pub GetIconState: unsafe extern "system" fn (this: *const nsIMozIconURI, aIconState: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString contentType; */
    pub GetContentType: unsafe extern "system" fn (this: *const nsIMozIconURI, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString fileExtension; */
    pub GetFileExtension: unsafe extern "system" fn (this: *const nsIMozIconURI, aFileExtension: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMozIconURI {


    /// `readonly attribute nsIURL iconURL;`
    #[inline]
    pub unsafe fn GetIconURL(&self, aIconURL: *mut *const nsIURL) -> ::nserror::nsresult {
        ((*self.vtable).GetIconURL)(self, aIconURL)
    }



    /// `readonly attribute unsigned long imageSize;`
    #[inline]
    pub unsafe fn GetImageSize(&self, aImageSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetImageSize)(self, aImageSize)
    }



    /// `readonly attribute ACString stockIcon;`
    #[inline]
    pub unsafe fn GetStockIcon(&self, aStockIcon: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetStockIcon)(self, aStockIcon)
    }



    /// `readonly attribute ACString iconSize;`
    #[inline]
    pub unsafe fn GetIconSize(&self, aIconSize: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetIconSize)(self, aIconSize)
    }



    /// `readonly attribute ACString iconState;`
    #[inline]
    pub unsafe fn GetIconState(&self, aIconState: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetIconState)(self, aIconState)
    }



    /// `readonly attribute ACString contentType;`
    #[inline]
    pub unsafe fn GetContentType(&self, aContentType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentType)(self, aContentType)
    }



    /// `readonly attribute ACString fileExtension;`
    #[inline]
    pub unsafe fn GetFileExtension(&self, aFileExtension: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFileExtension)(self, aFileExtension)
    }


}


