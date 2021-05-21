//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIWebBrowserFind.idl
//


/// `interface nsIWebBrowserFind : nsISupports`
///

/// ```text
/// /**
///  * nsIWebBrowserFind
///  *
///  * Searches for text in a web browser.
///  *
///  * Get one by doing a GetInterface on an nsIWebBrowser.
///  *
///  * By default, the implementation will search the focussed frame, or
///  * if there is no focussed frame, the web browser content area. It
///  * does not by default search subframes or iframes. To change this
///  * behaviour, and to explicitly set the frame to search,
///  * QueryInterface to nsIWebBrowserFindInFrames.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserFind {
    vtable: *const nsIWebBrowserFindVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserFind.
unsafe impl XpCom for nsIWebBrowserFind {
    const IID: nsIID = nsID(0xe4920136, 0xb3e0, 0x49e0,
        [0xb1, 0xcd, 0x6c, 0x78, 0x3d, 0x25, 0x91, 0xa8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserFind {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserFind.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserFindCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserFind`.
    fn coerce_from(v: &nsIWebBrowserFind) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserFindCoerce for nsIWebBrowserFind {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFind) -> &Self {
        v
    }
}

impl nsIWebBrowserFind {
    /// Cast this `nsIWebBrowserFind` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserFindCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserFind {
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
impl<T: nsISupportsCoerce> nsIWebBrowserFindCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFind) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserFind
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserFindVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean findNext (); */
    pub FindNext: unsafe extern "system" fn (this: *const nsIWebBrowserFind, _retval: *mut bool) -> ::nserror::nsresult,

    /* attribute AString searchString; */
    pub GetSearchString: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString searchString; */
    pub SetSearchString: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean findBackwards; */
    pub GetFindBackwards: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aFindBackwards: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean findBackwards; */
    pub SetFindBackwards: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aFindBackwards: bool) -> ::nserror::nsresult,

    /* attribute boolean wrapFind; */
    pub GetWrapFind: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aWrapFind: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean wrapFind; */
    pub SetWrapFind: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aWrapFind: bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub GetEntireWord: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aEntireWord: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub SetEntireWord: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aEntireWord: bool) -> ::nserror::nsresult,

    /* attribute boolean matchCase; */
    pub GetMatchCase: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aMatchCase: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean matchCase; */
    pub SetMatchCase: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aMatchCase: bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub GetMatchDiacritics: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aMatchDiacritics: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub SetMatchDiacritics: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aMatchDiacritics: bool) -> ::nserror::nsresult,

    /* attribute boolean searchFrames; */
    pub GetSearchFrames: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aSearchFrames: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean searchFrames; */
    pub SetSearchFrames: unsafe extern "system" fn (this: *const nsIWebBrowserFind, aSearchFrames: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserFind {

    /// ```text
    /// /**
    ///      * findNext
    ///      *
    ///      * Finds, highlights, and scrolls into view the next occurrence of the
    ///      * search string, using the current search settings. Fails if the
    ///      * search string is empty.
    ///      *
    ///      * @return  Whether an occurrence was found
    ///      */
    /// ```
    ///

    /// `boolean findNext ();`
    #[inline]
    pub unsafe fn FindNext(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).FindNext)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * searchString
    ///      *
    ///      * The string to search for. This must be non-empty to search.
    ///      */
    /// ```
    ///

    /// `attribute AString searchString;`
    #[inline]
    pub unsafe fn GetSearchString(&self, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchString)(self, aSearchString)
    }


    /// ```text
    /// /**
    ///      * searchString
    ///      *
    ///      * The string to search for. This must be non-empty to search.
    ///      */
    /// ```
    ///

    /// `attribute AString searchString;`
    #[inline]
    pub unsafe fn SetSearchString(&self, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchString)(self, aSearchString)
    }


    /// ```text
    /// /**
    ///      * findBackwards
    ///      *
    ///      * Whether to find backwards (towards the beginning of the document).
    ///      * Default is false (search forward).
    ///      */
    /// ```
    ///

    /// `attribute boolean findBackwards;`
    #[inline]
    pub unsafe fn GetFindBackwards(&self, aFindBackwards: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetFindBackwards)(self, aFindBackwards)
    }


    /// ```text
    /// /**
    ///      * findBackwards
    ///      *
    ///      * Whether to find backwards (towards the beginning of the document).
    ///      * Default is false (search forward).
    ///      */
    /// ```
    ///

    /// `attribute boolean findBackwards;`
    #[inline]
    pub unsafe fn SetFindBackwards(&self, aFindBackwards: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetFindBackwards)(self, aFindBackwards)
    }


    /// ```text
    /// /**
    ///      * wrapFind
    ///      *
    ///      * Whether the search wraps around to the start (or end) of the document
    ///      * if no match was found between the current position and the end (or
        ///      * beginning). Works correctly when searching backwards. Default is
    ///      * false.
    ///      */
    /// ```
    ///

    /// `attribute boolean wrapFind;`
    #[inline]
    pub unsafe fn GetWrapFind(&self, aWrapFind: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWrapFind)(self, aWrapFind)
    }


    /// ```text
    /// /**
    ///      * wrapFind
    ///      *
    ///      * Whether the search wraps around to the start (or end) of the document
    ///      * if no match was found between the current position and the end (or
        ///      * beginning). Works correctly when searching backwards. Default is
    ///      * false.
    ///      */
    /// ```
    ///

    /// `attribute boolean wrapFind;`
    #[inline]
    pub unsafe fn SetWrapFind(&self, aWrapFind: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetWrapFind)(self, aWrapFind)
    }


    /// ```text
    /// /**
    ///      * entireWord
    ///      *
    ///      * Whether to match entire words only. Default is false.
    ///      */
    /// ```
    ///

    /// `attribute boolean entireWord;`
    #[inline]
    pub unsafe fn GetEntireWord(&self, aEntireWord: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEntireWord)(self, aEntireWord)
    }


    /// ```text
    /// /**
    ///      * entireWord
    ///      *
    ///      * Whether to match entire words only. Default is false.
    ///      */
    /// ```
    ///

    /// `attribute boolean entireWord;`
    #[inline]
    pub unsafe fn SetEntireWord(&self, aEntireWord: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEntireWord)(self, aEntireWord)
    }


    /// ```text
    /// /**
    ///      * matchCase
    ///      *
    ///      * Whether to match case (case sensitive) when searching. Default is false.
    ///      */
    /// ```
    ///

    /// `attribute boolean matchCase;`
    #[inline]
    pub unsafe fn GetMatchCase(&self, aMatchCase: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchCase)(self, aMatchCase)
    }


    /// ```text
    /// /**
    ///      * matchCase
    ///      *
    ///      * Whether to match case (case sensitive) when searching. Default is false.
    ///      */
    /// ```
    ///

    /// `attribute boolean matchCase;`
    #[inline]
    pub unsafe fn SetMatchCase(&self, aMatchCase: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetMatchCase)(self, aMatchCase)
    }


    /// ```text
    /// /**
    ///      * matchDiacritics
    ///      *
    ///      * Whether to match diacritics when searching. Default is false.
    ///      */
    /// ```
    ///

    /// `attribute boolean matchDiacritics;`
    #[inline]
    pub unsafe fn GetMatchDiacritics(&self, aMatchDiacritics: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchDiacritics)(self, aMatchDiacritics)
    }


    /// ```text
    /// /**
    ///      * matchDiacritics
    ///      *
    ///      * Whether to match diacritics when searching. Default is false.
    ///      */
    /// ```
    ///

    /// `attribute boolean matchDiacritics;`
    #[inline]
    pub unsafe fn SetMatchDiacritics(&self, aMatchDiacritics: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetMatchDiacritics)(self, aMatchDiacritics)
    }


    /// ```text
    /// /**
    ///      * searchFrames
    ///      *
    ///      * Whether to search through all frames in the content area. Default is true.
    ///      *
    ///      * Note that you can control whether the search propagates into child or
    ///      * parent frames explicitly using nsIWebBrowserFindInFrames, but if one,
    ///      * but not both, of searchSubframes and searchParentFrames are set, this
    ///      * returns false.
    ///      */
    /// ```
    ///

    /// `attribute boolean searchFrames;`
    #[inline]
    pub unsafe fn GetSearchFrames(&self, aSearchFrames: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchFrames)(self, aSearchFrames)
    }


    /// ```text
    /// /**
    ///      * searchFrames
    ///      *
    ///      * Whether to search through all frames in the content area. Default is true.
    ///      *
    ///      * Note that you can control whether the search propagates into child or
    ///      * parent frames explicitly using nsIWebBrowserFindInFrames, but if one,
    ///      * but not both, of searchSubframes and searchParentFrames are set, this
    ///      * returns false.
    ///      */
    /// ```
    ///

    /// `attribute boolean searchFrames;`
    #[inline]
    pub unsafe fn SetSearchFrames(&self, aSearchFrames: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchFrames)(self, aSearchFrames)
    }


}


/// `interface nsIWebBrowserFindInFrames : nsISupports`
///

/// ```text
/// /**
///  * nsIWebBrowserFindInFrames
///  *
///  * Controls how find behaves when multiple frames or iframes are present.
///  *
///  * Get by doing a QueryInterface from nsIWebBrowserFind.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserFindInFrames {
    vtable: *const nsIWebBrowserFindInFramesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserFindInFrames.
unsafe impl XpCom for nsIWebBrowserFindInFrames {
    const IID: nsIID = nsID(0xe0f5d182, 0x34bc, 0x11d5,
        [0xbe, 0x5b, 0xb7, 0x60, 0x67, 0x6c, 0x6e, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserFindInFrames {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserFindInFrames.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserFindInFramesCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserFindInFrames`.
    fn coerce_from(v: &nsIWebBrowserFindInFrames) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserFindInFramesCoerce for nsIWebBrowserFindInFrames {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFindInFrames) -> &Self {
        v
    }
}

impl nsIWebBrowserFindInFrames {
    /// Cast this `nsIWebBrowserFindInFrames` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserFindInFramesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserFindInFrames {
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
impl<T: nsISupportsCoerce> nsIWebBrowserFindInFramesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFindInFrames) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserFindInFrames
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserFindInFramesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute mozIDOMWindowProxy currentSearchFrame; */
    pub GetCurrentSearchFrame: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aCurrentSearchFrame: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* attribute mozIDOMWindowProxy currentSearchFrame; */
    pub SetCurrentSearchFrame: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aCurrentSearchFrame: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* attribute mozIDOMWindowProxy rootSearchFrame; */
    pub GetRootSearchFrame: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aRootSearchFrame: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* attribute mozIDOMWindowProxy rootSearchFrame; */
    pub SetRootSearchFrame: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aRootSearchFrame: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* attribute boolean searchSubframes; */
    pub GetSearchSubframes: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aSearchSubframes: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean searchSubframes; */
    pub SetSearchSubframes: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aSearchSubframes: bool) -> ::nserror::nsresult,

    /* attribute boolean searchParentFrames; */
    pub GetSearchParentFrames: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aSearchParentFrames: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean searchParentFrames; */
    pub SetSearchParentFrames: unsafe extern "system" fn (this: *const nsIWebBrowserFindInFrames, aSearchParentFrames: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserFindInFrames {

    /// ```text
    /// /**
    ///      * currentSearchFrame
    ///      *
    ///      * Frame at which to start the search. Once the search is done, this will
    ///      * be set to be the last frame searched, whether or not a result was found.
    ///      * Has to be equal to or contained within the rootSearchFrame.
    ///      */
    /// ```
    ///

    /// `attribute mozIDOMWindowProxy currentSearchFrame;`
    #[inline]
    pub unsafe fn GetCurrentSearchFrame(&self, aCurrentSearchFrame: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentSearchFrame)(self, aCurrentSearchFrame)
    }


    /// ```text
    /// /**
    ///      * currentSearchFrame
    ///      *
    ///      * Frame at which to start the search. Once the search is done, this will
    ///      * be set to be the last frame searched, whether or not a result was found.
    ///      * Has to be equal to or contained within the rootSearchFrame.
    ///      */
    /// ```
    ///

    /// `attribute mozIDOMWindowProxy currentSearchFrame;`
    #[inline]
    pub unsafe fn SetCurrentSearchFrame(&self, aCurrentSearchFrame: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentSearchFrame)(self, aCurrentSearchFrame)
    }


    /// ```text
    /// /**
    ///      * rootSearchFrame
    ///      *
    ///      * Frame within which to confine the search (normally the content area frame).
    ///      * Set this to only search a subtree of the frame hierarchy.
    ///      */
    /// ```
    ///

    /// `attribute mozIDOMWindowProxy rootSearchFrame;`
    #[inline]
    pub unsafe fn GetRootSearchFrame(&self, aRootSearchFrame: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetRootSearchFrame)(self, aRootSearchFrame)
    }


    /// ```text
    /// /**
    ///      * rootSearchFrame
    ///      *
    ///      * Frame within which to confine the search (normally the content area frame).
    ///      * Set this to only search a subtree of the frame hierarchy.
    ///      */
    /// ```
    ///

    /// `attribute mozIDOMWindowProxy rootSearchFrame;`
    #[inline]
    pub unsafe fn SetRootSearchFrame(&self, aRootSearchFrame: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).SetRootSearchFrame)(self, aRootSearchFrame)
    }


    /// ```text
    /// /**
    ///      * searchSubframes
    ///      *
    ///      * Whether to recurse down into subframes while searching. Default is true.
    ///      *
    ///      * Setting nsIWebBrowserfind.searchFrames to true sets this to true.
    ///      */
    /// ```
    ///

    /// `attribute boolean searchSubframes;`
    #[inline]
    pub unsafe fn GetSearchSubframes(&self, aSearchSubframes: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchSubframes)(self, aSearchSubframes)
    }


    /// ```text
    /// /**
    ///      * searchSubframes
    ///      *
    ///      * Whether to recurse down into subframes while searching. Default is true.
    ///      *
    ///      * Setting nsIWebBrowserfind.searchFrames to true sets this to true.
    ///      */
    /// ```
    ///

    /// `attribute boolean searchSubframes;`
    #[inline]
    pub unsafe fn SetSearchSubframes(&self, aSearchSubframes: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchSubframes)(self, aSearchSubframes)
    }


    /// ```text
    /// /**
    ///      * searchParentFrames
    ///      *
    ///      * Whether to allow the search to propagate out of the currentSearchFrame into its
    ///      * parent frame(s). Search is always confined within the rootSearchFrame. Default
    ///      * is true.
    ///      *
    ///      * Setting nsIWebBrowserfind.searchFrames to true sets this to true.
    ///      */
    /// ```
    ///

    /// `attribute boolean searchParentFrames;`
    #[inline]
    pub unsafe fn GetSearchParentFrames(&self, aSearchParentFrames: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchParentFrames)(self, aSearchParentFrames)
    }


    /// ```text
    /// /**
    ///      * searchParentFrames
    ///      *
    ///      * Whether to allow the search to propagate out of the currentSearchFrame into its
    ///      * parent frame(s). Search is always confined within the rootSearchFrame. Default
    ///      * is true.
    ///      *
    ///      * Setting nsIWebBrowserfind.searchFrames to true sets this to true.
    ///      */
    /// ```
    ///

    /// `attribute boolean searchParentFrames;`
    #[inline]
    pub unsafe fn SetSearchParentFrames(&self, aSearchParentFrames: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchParentFrames)(self, aSearchParentFrames)
    }


}


