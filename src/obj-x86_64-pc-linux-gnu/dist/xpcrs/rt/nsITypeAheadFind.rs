//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/typeaheadfind/nsITypeAheadFind.idl
//


/// `interface nsITypeAheadFind : nsISupports`
///

/// ```text
/// /****************************** nsTypeAheadFind ******************************/
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITypeAheadFind {
    vtable: *const nsITypeAheadFindVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITypeAheadFind.
unsafe impl XpCom for nsITypeAheadFind {
    const IID: nsIID = nsID(0xae501e28, 0xc57f, 0x4692,
        [0xac, 0x74, 0x41, 0x0e, 0x1b, 0xed, 0x98, 0xb7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITypeAheadFind {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITypeAheadFind.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITypeAheadFindCoerce {
    /// Cheaply cast a value of this type from a `nsITypeAheadFind`.
    fn coerce_from(v: &nsITypeAheadFind) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITypeAheadFindCoerce for nsITypeAheadFind {
    #[inline]
    fn coerce_from(v: &nsITypeAheadFind) -> &Self {
        v
    }
}

impl nsITypeAheadFind {
    /// Cast this `nsITypeAheadFind` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITypeAheadFindCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITypeAheadFind {
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
impl<T: nsISupportsCoerce> nsITypeAheadFindCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITypeAheadFind) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITypeAheadFind
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITypeAheadFindVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in nsIDocShell aDocShell); */
    pub Init: unsafe extern "system" fn (this: *const nsITypeAheadFind, aDocShell: *const nsIDocShell) -> ::nserror::nsresult,

    /* unsigned short find (in AString aSearchString, in boolean aLinksOnly, in unsigned long aMode, in boolean aDontIterateFrames); */
    pub Find: unsafe extern "system" fn (this: *const nsITypeAheadFind, aSearchString: *const ::nsstring::nsAString, aLinksOnly: bool, aMode: u32, aDontIterateFrames: bool, _retval: *mut u16) -> ::nserror::nsresult,

    /* Range getFoundRange (); */
    pub GetFoundRange: unsafe extern "system" fn (this: *const nsITypeAheadFind, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void setDocShell (in nsIDocShell aDocShell); */
    pub SetDocShell: unsafe extern "system" fn (this: *const nsITypeAheadFind, aDocShell: *const nsIDocShell) -> ::nserror::nsresult,

    /* void setSelectionModeAndRepaint (in short toggle); */
    pub SetSelectionModeAndRepaint: unsafe extern "system" fn (this: *const nsITypeAheadFind, toggle: i16) -> ::nserror::nsresult,

    /* void collapseSelection (); */
    pub CollapseSelection: unsafe extern "system" fn (this: *const nsITypeAheadFind) -> ::nserror::nsresult,

    /* boolean isRangeVisible (in Range aRange, in boolean aMustBeInViewPort); */
    pub IsRangeVisible: unsafe extern "system" fn (this: *const nsITypeAheadFind, aRange: *const libc::c_void, aMustBeInViewPort: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isRangeRendered (in Range aRange); */
    pub IsRangeRendered: unsafe extern "system" fn (this: *const nsITypeAheadFind, aRange: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString searchString; */
    pub GetSearchString: unsafe extern "system" fn (this: *const nsITypeAheadFind, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean caseSensitive; */
    pub GetCaseSensitive: unsafe extern "system" fn (this: *const nsITypeAheadFind, aCaseSensitive: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean caseSensitive; */
    pub SetCaseSensitive: unsafe extern "system" fn (this: *const nsITypeAheadFind, aCaseSensitive: bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub GetMatchDiacritics: unsafe extern "system" fn (this: *const nsITypeAheadFind, aMatchDiacritics: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub SetMatchDiacritics: unsafe extern "system" fn (this: *const nsITypeAheadFind, aMatchDiacritics: bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub GetEntireWord: unsafe extern "system" fn (this: *const nsITypeAheadFind, aEntireWord: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub SetEntireWord: unsafe extern "system" fn (this: *const nsITypeAheadFind, aEntireWord: bool) -> ::nserror::nsresult,

    /* readonly attribute Element foundLink; */
    pub GetFoundLink: unsafe extern "system" fn (this: *const nsITypeAheadFind, aFoundLink: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute Element foundEditable; */
    pub GetFoundEditable: unsafe extern "system" fn (this: *const nsITypeAheadFind, aFoundEditable: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindow currentWindow; */
    pub GetCurrentWindow: unsafe extern "system" fn (this: *const nsITypeAheadFind, aCurrentWindow: *mut*const mozIDOMWindow) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITypeAheadFind {
    /// ```text
    /// /******************************* Constants *******************************/
    /// ```
    ///

    pub const FIND_INITIAL: i64 = 0;


    pub const FIND_NEXT: i64 = 1;


    pub const FIND_PREVIOUS: i64 = 2;


    pub const FIND_FIRST: i64 = 3;


    pub const FIND_LAST: i64 = 4;


    pub const FIND_FOUND: i64 = 0;


    pub const FIND_NOTFOUND: i64 = 1;


    pub const FIND_WRAPPED: i64 = 2;


    pub const FIND_PENDING: i64 = 3;

    /// ```text
    /// /****************************** Initializer ******************************/
    /// ```
    ///

    /// `void init (in nsIDocShell aDocShell);`
    #[inline]
    pub unsafe fn Init(&self, aDocShell: *const nsIDocShell) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aDocShell)
    }


    /// ```text
    /// /***************************** Core functions ****************************/
    /// ```
    ///

    /// `unsigned short find (in AString aSearchString, in boolean aLinksOnly, in unsigned long aMode, in boolean aDontIterateFrames);`
    #[inline]
    pub unsafe fn Find(&self, aSearchString: *const ::nsstring::nsAString, aLinksOnly: bool, aMode: u32, aDontIterateFrames: bool, _retval: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).Find)(self, aSearchString, aLinksOnly, aMode, aDontIterateFrames, _retval)
    }



    /// `Range getFoundRange ();`
    #[inline]
    pub unsafe fn GetFoundRange(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFoundRange)(self, _retval)
    }


    /// ```text
    /// /**************************** Helper functions ***************************/
    /// ```
    ///

    /// `void setDocShell (in nsIDocShell aDocShell);`
    #[inline]
    pub unsafe fn SetDocShell(&self, aDocShell: *const nsIDocShell) -> ::nserror::nsresult {
        ((*self.vtable).SetDocShell)(self, aDocShell)
    }



    /// `void setSelectionModeAndRepaint (in short toggle);`
    #[inline]
    pub unsafe fn SetSelectionModeAndRepaint(&self, toggle: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetSelectionModeAndRepaint)(self, toggle)
    }



    /// `void collapseSelection ();`
    #[inline]
    pub unsafe fn CollapseSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CollapseSelection)(self, )
    }



    /// `boolean isRangeVisible (in Range aRange, in boolean aMustBeInViewPort);`
    #[inline]
    pub unsafe fn IsRangeVisible(&self, aRange: *const libc::c_void, aMustBeInViewPort: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsRangeVisible)(self, aRange, aMustBeInViewPort, _retval)
    }



    /// `boolean isRangeRendered (in Range aRange);`
    #[inline]
    pub unsafe fn IsRangeRendered(&self, aRange: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsRangeRendered)(self, aRange, _retval)
    }


    /// ```text
    /// /******************************* Attributes ******************************/
    /// ```
    ///

    /// `readonly attribute AString searchString;`
    #[inline]
    pub unsafe fn GetSearchString(&self, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchString)(self, aSearchString)
    }



    /// `attribute boolean caseSensitive;`
    #[inline]
    pub unsafe fn GetCaseSensitive(&self, aCaseSensitive: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCaseSensitive)(self, aCaseSensitive)
    }



    /// `attribute boolean caseSensitive;`
    #[inline]
    pub unsafe fn SetCaseSensitive(&self, aCaseSensitive: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCaseSensitive)(self, aCaseSensitive)
    }



    /// `attribute boolean matchDiacritics;`
    #[inline]
    pub unsafe fn GetMatchDiacritics(&self, aMatchDiacritics: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchDiacritics)(self, aMatchDiacritics)
    }



    /// `attribute boolean matchDiacritics;`
    #[inline]
    pub unsafe fn SetMatchDiacritics(&self, aMatchDiacritics: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetMatchDiacritics)(self, aMatchDiacritics)
    }



    /// `attribute boolean entireWord;`
    #[inline]
    pub unsafe fn GetEntireWord(&self, aEntireWord: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEntireWord)(self, aEntireWord)
    }



    /// `attribute boolean entireWord;`
    #[inline]
    pub unsafe fn SetEntireWord(&self, aEntireWord: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEntireWord)(self, aEntireWord)
    }



    /// `readonly attribute Element foundLink;`
    #[inline]
    pub unsafe fn GetFoundLink(&self, aFoundLink: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFoundLink)(self, aFoundLink)
    }



    /// `readonly attribute Element foundEditable;`
    #[inline]
    pub unsafe fn GetFoundEditable(&self, aFoundEditable: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFoundEditable)(self, aFoundEditable)
    }



    /// `readonly attribute mozIDOMWindow currentWindow;`
    #[inline]
    pub unsafe fn GetCurrentWindow(&self, aCurrentWindow: *mut*const mozIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentWindow)(self, aCurrentWindow)
    }


}


