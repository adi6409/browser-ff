//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIPromptCollection.idl
//


/// `interface nsIPromptCollection : nsISupports`
///

/// ```text
/// /**
///  * This interface contains various specialized prompts that the app can
///  * implement.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPromptCollection {
    vtable: *const nsIPromptCollectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPromptCollection.
unsafe impl XpCom for nsIPromptCollection {
    const IID: nsIID = nsID(0x7913837c, 0x9623, 0x11ea,
        [0xbb, 0x37, 0x02, 0x42, 0xac, 0x13, 0x00, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPromptCollection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPromptCollection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPromptCollectionCoerce {
    /// Cheaply cast a value of this type from a `nsIPromptCollection`.
    fn coerce_from(v: &nsIPromptCollection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPromptCollectionCoerce for nsIPromptCollection {
    #[inline]
    fn coerce_from(v: &nsIPromptCollection) -> &Self {
        v
    }
}

impl nsIPromptCollection {
    /// Cast this `nsIPromptCollection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPromptCollectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPromptCollection {
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
impl<T: nsISupportsCoerce> nsIPromptCollectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPromptCollection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPromptCollection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPromptCollectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Promise asyncBeforeUnloadCheck (in BrowsingContext aBrowsingContext); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncBeforeUnloadCheck: *const ::libc::c_void,

    /* boolean confirmRepost (in BrowsingContext aBrowsingContext); */
    pub ConfirmRepost: unsafe extern "system" fn (this: *const nsIPromptCollection, aBrowsingContext: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean confirmFolderUpload (in BrowsingContext aBrowsingContext, in AString aDirectoryName); */
    pub ConfirmFolderUpload: unsafe extern "system" fn (this: *const nsIPromptCollection, aBrowsingContext: *const libc::c_void, aDirectoryName: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPromptCollection {

    /// ```text
    /// /**
    ///    * Puts up a dialog for the before unload prompt.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    *
    ///    * @return Promise which resolves to true if the page should be allowed to
    ///    * navigate away
    ///    */
    /// ```
    ///

    /// `Promise asyncBeforeUnloadCheck (in BrowsingContext aBrowsingContext);`
    const _AsyncBeforeUnloadCheck: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog for the confirm repost prompt.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    *
    ///    * @return true if the page should be allowed to repost data.
    ///    */
    /// ```
    ///

    /// `boolean confirmRepost (in BrowsingContext aBrowsingContext);`
    #[inline]
    pub unsafe fn ConfirmRepost(&self, aBrowsingContext: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmRepost)(self, aBrowsingContext, _retval)
    }


    /// ```text
    /// /**
    ///    * Ask the user for confirmation to upload a selected folder.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param aDirectoryName
    ///    *        Name of the folder that will be uploaded.
    ///    *
    ///    * @return true if the user confirmed the upload, false otherwise.
    ///    */
    /// ```
    ///

    /// `boolean confirmFolderUpload (in BrowsingContext aBrowsingContext, in AString aDirectoryName);`
    #[inline]
    pub unsafe fn ConfirmFolderUpload(&self, aBrowsingContext: *const libc::c_void, aDirectoryName: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmFolderUpload)(self, aBrowsingContext, aDirectoryName, _retval)
    }


}


