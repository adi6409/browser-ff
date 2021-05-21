//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserPrint.idl
//


/// `interface nsIWebBrowserPrint : nsISupports`
///

/// ```text
/// /**
///  * nsIWebBrowserPrint corresponds to the main interface
///  * for printing an embedded Gecko web browser window/document
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserPrint {
    vtable: *const nsIWebBrowserPrintVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserPrint.
unsafe impl XpCom for nsIWebBrowserPrint {
    const IID: nsIID = nsID(0xc9a934ed, 0xfff1, 0x4971,
        [0xbf, 0xba, 0x6c, 0x25, 0xad, 0x70, 0xe1, 0xe6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserPrint {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserPrint.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserPrintCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserPrint`.
    fn coerce_from(v: &nsIWebBrowserPrint) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserPrintCoerce for nsIWebBrowserPrint {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPrint) -> &Self {
        v
    }
}

impl nsIWebBrowserPrint {
    /// Cast this `nsIWebBrowserPrint` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserPrintCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserPrint {
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
impl<T: nsISupportsCoerce> nsIWebBrowserPrintCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPrint) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserPrint
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserPrintVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrintSettings currentPrintSettings; */
    pub GetCurrentPrintSettings: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aCurrentPrintSettings: *mut*const nsIPrintSettings) -> ::nserror::nsresult,

    /* readonly attribute boolean doingPrint; */
    pub GetDoingPrint: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aDoingPrint: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean doingPrintPreview; */
    pub GetDoingPrintPreview: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aDoingPrintPreview: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long rawNumPages; */
    pub GetRawNumPages: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aRawNumPages: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long printPreviewNumPages; */
    pub GetPrintPreviewNumPages: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aPrintPreviewNumPages: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long printPreviewCurrentPageNumber; */
    pub GetPrintPreviewCurrentPageNumber: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aPrintPreviewCurrentPageNumber: *mut i32) -> ::nserror::nsresult,

    /* [noscript] void print (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener); */
    pub Print: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aThePrintSettings: *const nsIPrintSettings, aWPListener: *const nsIWebProgressListener) -> ::nserror::nsresult,

    /* [noscript] void printPreview (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener, in PrintPreviewResolver aCallback); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub PrintPreview: *const ::libc::c_void,

    /* void printPreviewScrollToPage (in short aNavType, in long aPageNum); */
    pub PrintPreviewScrollToPage: unsafe extern "system" fn (this: *const nsIWebBrowserPrint, aNavType: i16, aPageNum: i32) -> ::nserror::nsresult,

    /* void cancel (); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIWebBrowserPrint) -> ::nserror::nsresult,

    /* void exitPrintPreview (); */
    pub ExitPrintPreview: unsafe extern "system" fn (this: *const nsIWebBrowserPrint) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserPrint {
    /// ```text
    /// /**
    ///    * PrintPreview Navigation Constants
    ///    *
    ///    * XXXdholbert Consider renaming these? Strictly speaking, these deal with
    ///    * *sheets* (which are roughly the same as pages in the default configuration
        ///    * of one page per sheet). Fix in bug 1669762.
    ///    */
    /// ```
    ///

    pub const PRINTPREVIEW_GOTO_PAGENUM: i64 = 0;


    pub const PRINTPREVIEW_PREV_PAGE: i64 = 1;


    pub const PRINTPREVIEW_NEXT_PAGE: i64 = 2;


    pub const PRINTPREVIEW_HOME: i64 = 3;


    pub const PRINTPREVIEW_END: i64 = 4;

    /// ```text
    /// /**
    ///    * Returns a pointer to the PrintSettings object that
    ///    * that was passed into either "print" or "print preview"
    ///    *
    ///    * This enables any consumers of the interface to have access
    ///    * to the "current" PrintSetting at later points in the execution
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPrintSettings currentPrintSettings;`
    #[inline]
    pub unsafe fn GetCurrentPrintSettings(&self, aCurrentPrintSettings: *mut*const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentPrintSettings)(self, aCurrentPrintSettings)
    }


    /// ```text
    /// /**
    ///    * Returns whether it is in Print mode
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean doingPrint;`
    #[inline]
    pub unsafe fn GetDoingPrint(&self, aDoingPrint: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDoingPrint)(self, aDoingPrint)
    }


    /// ```text
    /// /**
    ///    * Returns whether it is in Print Preview mode
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean doingPrintPreview;`
    #[inline]
    pub unsafe fn GetDoingPrintPreview(&self, aDoingPrintPreview: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDoingPrintPreview)(self, aDoingPrintPreview)
    }


    /// ```text
    /// /**
    ///    * This represents the "raw" total number of pages, where "raw" means that
    ///    * this value is *not amended* to account for reductions from pages-per-sheet
    ///    * or page ranges (unlike other APIs on this interface).
    ///    *
    ///    * So e.g. for a 20-page document, this attribute will be 20, regardless of
    ///    * whether the user has chosen a smaller page range, and regardless of
    ///    * whether the user is using pages-per-sheet to reduce the number of sheets.
    ///    */
    /// ```
    ///

    /// `readonly attribute long rawNumPages;`
    #[inline]
    pub unsafe fn GetRawNumPages(&self, aRawNumPages: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRawNumPages)(self, aRawNumPages)
    }


    /// ```text
    /// /**
    ///    * This returns the total number of pages for the Print Preview
    ///    *
    ///    * XXXdholbert Consider renaming this? Strictly speaking, this is the number
    ///    * of *sheets* (which is the same as the number of pages in the default
        ///    * configuration of one page per sheet). Fix in bug 1669762.
    ///    */
    /// ```
    ///

    /// `readonly attribute long printPreviewNumPages;`
    #[inline]
    pub unsafe fn GetPrintPreviewNumPages(&self, aPrintPreviewNumPages: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPrintPreviewNumPages)(self, aPrintPreviewNumPages)
    }


    /// ```text
    /// /**
    ///    * This returns the number of the page which is currently in the Print Preview viewport
    ///    *
    ///    * XXXdholbert Consider renaming this? (similar to printPreviewNumPages above)
    ///    * Strictly speaking, this is the number of the *sheet* which is currently in
    ///    * the print preview viewport. Fix in bug 1669762.
    ///    */
    /// ```
    ///

    /// `readonly attribute long printPreviewCurrentPageNumber;`
    #[inline]
    pub unsafe fn GetPrintPreviewCurrentPageNumber(&self, aPrintPreviewCurrentPageNumber: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPrintPreviewCurrentPageNumber)(self, aPrintPreviewCurrentPageNumber)
    }


    /// ```text
    /// /**
    ///    * Print the specified DOM window
    ///    *
    ///    * @param aThePrintSettings - Printer Settings for the print job, if aThePrintSettings is null
    ///    *                            then the global PS will be used.
    ///    * @param aWPListener - is updated during the print
    ///    * @return void
    ///    */
    /// ```
    ///

    /// `[noscript] void print (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener);`
    #[inline]
    pub unsafe fn Print(&self, aThePrintSettings: *const nsIPrintSettings, aWPListener: *const nsIWebProgressListener) -> ::nserror::nsresult {
        ((*self.vtable).Print)(self, aThePrintSettings, aWPListener)
    }


    /// ```text
    /// /**
    ///    * Print Preview the specified DOM window
    ///    *
    ///    * @param aThePrintSettings - Printer Settings for the print preview, if aThePrintSettings is null
    ///    *                            then the global PS will be used.
    ///    * @param aWPListener - is updated during the printpreview
    ///    * @return void
    ///    */
    /// ```
    ///

    /// `[noscript] void printPreview (in nsIPrintSettings aThePrintSettings, in nsIWebProgressListener aWPListener, in PrintPreviewResolver aCallback);`
    const _PrintPreview: () = ();

    /// ```text
    /// /**
    ///    * @param aNavType - navigation enum
    ///    * @param aPageNum - page num to navigate to when aNavType = ePrintPreviewGoToPageNum
    ///    * @return void
    ///    */
    /// ```
    ///

    /// `void printPreviewScrollToPage (in short aNavType, in long aPageNum);`
    #[inline]
    pub unsafe fn PrintPreviewScrollToPage(&self, aNavType: i16, aPageNum: i32) -> ::nserror::nsresult {
        ((*self.vtable).PrintPreviewScrollToPage)(self, aNavType, aPageNum)
    }


    /// ```text
    /// /**
    ///    * Cancels the current print
    ///    * @return void
    ///    */
    /// ```
    ///

    /// `void cancel ();`
    #[inline]
    pub unsafe fn Cancel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, )
    }


    /// ```text
    /// /**
    ///    * This exists PrintPreview mode and returns browser window to galley mode
    ///    * @return void
    ///    */
    /// ```
    ///

    /// `void exitPrintPreview ();`
    #[inline]
    pub unsafe fn ExitPrintPreview(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ExitPrintPreview)(self, )
    }


}


