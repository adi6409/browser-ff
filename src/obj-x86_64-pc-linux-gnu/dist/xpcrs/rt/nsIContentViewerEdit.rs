//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIContentViewerEdit.idl
//


/// `interface nsIContentViewerEdit : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentViewerEdit {
    vtable: *const nsIContentViewerEditVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentViewerEdit.
unsafe impl XpCom for nsIContentViewerEdit {
    const IID: nsIID = nsID(0x35be2d7e, 0xf29b, 0x48ec,
        [0xbf, 0x7e, 0x80, 0xa3, 0x0a, 0x72, 0x4d, 0xe3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentViewerEdit {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentViewerEdit.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentViewerEditCoerce {
    /// Cheaply cast a value of this type from a `nsIContentViewerEdit`.
    fn coerce_from(v: &nsIContentViewerEdit) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentViewerEditCoerce for nsIContentViewerEdit {
    #[inline]
    fn coerce_from(v: &nsIContentViewerEdit) -> &Self {
        v
    }
}

impl nsIContentViewerEdit {
    /// Cast this `nsIContentViewerEdit` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentViewerEditCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentViewerEdit {
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
impl<T: nsISupportsCoerce> nsIContentViewerEditCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentViewerEdit) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentViewerEdit
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentViewerEditVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void clearSelection (); */
    pub ClearSelection: unsafe extern "system" fn (this: *const nsIContentViewerEdit) -> ::nserror::nsresult,

    /* void selectAll (); */
    pub SelectAll: unsafe extern "system" fn (this: *const nsIContentViewerEdit) -> ::nserror::nsresult,

    /* void copySelection (); */
    pub CopySelection: unsafe extern "system" fn (this: *const nsIContentViewerEdit) -> ::nserror::nsresult,

    /* readonly attribute boolean copyable; */
    pub GetCopyable: unsafe extern "system" fn (this: *const nsIContentViewerEdit, aCopyable: *mut bool) -> ::nserror::nsresult,

    /* void copyLinkLocation (); */
    pub CopyLinkLocation: unsafe extern "system" fn (this: *const nsIContentViewerEdit) -> ::nserror::nsresult,

    /* readonly attribute boolean inLink; */
    pub GetInLink: unsafe extern "system" fn (this: *const nsIContentViewerEdit, aInLink: *mut bool) -> ::nserror::nsresult,

    /* void copyImage (in long aCopyFlags); */
    pub CopyImage: unsafe extern "system" fn (this: *const nsIContentViewerEdit, aCopyFlags: i32) -> ::nserror::nsresult,

    /* readonly attribute boolean inImage; */
    pub GetInImage: unsafe extern "system" fn (this: *const nsIContentViewerEdit, aInImage: *mut bool) -> ::nserror::nsresult,

    /* AString getContents (in string aMimeType, in boolean aSelectionOnly); */
    pub GetContents: unsafe extern "system" fn (this: *const nsIContentViewerEdit, aMimeType: *const libc::c_char, aSelectionOnly: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean canGetContents; */
    pub GetCanGetContents: unsafe extern "system" fn (this: *const nsIContentViewerEdit, aCanGetContents: *mut bool) -> ::nserror::nsresult,

    /* void setCommandNode (in Node aNode); */
    pub SetCommandNode: unsafe extern "system" fn (this: *const nsIContentViewerEdit, aNode: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentViewerEdit {

    pub const COPY_IMAGE_TEXT: i64 = 1;


    pub const COPY_IMAGE_HTML: i64 = 2;


    pub const COPY_IMAGE_DATA: i64 = 4;


    pub const COPY_IMAGE_ALL: i64 = -1;


    /// `void clearSelection ();`
    #[inline]
    pub unsafe fn ClearSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearSelection)(self, )
    }



    /// `void selectAll ();`
    #[inline]
    pub unsafe fn SelectAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectAll)(self, )
    }



    /// `void copySelection ();`
    #[inline]
    pub unsafe fn CopySelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CopySelection)(self, )
    }



    /// `readonly attribute boolean copyable;`
    #[inline]
    pub unsafe fn GetCopyable(&self, aCopyable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCopyable)(self, aCopyable)
    }



    /// `void copyLinkLocation ();`
    #[inline]
    pub unsafe fn CopyLinkLocation(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CopyLinkLocation)(self, )
    }



    /// `readonly attribute boolean inLink;`
    #[inline]
    pub unsafe fn GetInLink(&self, aInLink: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInLink)(self, aInLink)
    }



    /// `void copyImage (in long aCopyFlags);`
    #[inline]
    pub unsafe fn CopyImage(&self, aCopyFlags: i32) -> ::nserror::nsresult {
        ((*self.vtable).CopyImage)(self, aCopyFlags)
    }



    /// `readonly attribute boolean inImage;`
    #[inline]
    pub unsafe fn GetInImage(&self, aInImage: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInImage)(self, aInImage)
    }



    /// `AString getContents (in string aMimeType, in boolean aSelectionOnly);`
    #[inline]
    pub unsafe fn GetContents(&self, aMimeType: *const libc::c_char, aSelectionOnly: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetContents)(self, aMimeType, aSelectionOnly, _retval)
    }



    /// `readonly attribute boolean canGetContents;`
    #[inline]
    pub unsafe fn GetCanGetContents(&self, aCanGetContents: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanGetContents)(self, aCanGetContents)
    }



    /// `void setCommandNode (in Node aNode);`
    #[inline]
    pub unsafe fn SetCommandNode(&self, aNode: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetCommandNode)(self, aNode)
    }


}


