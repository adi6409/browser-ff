//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIFind.idl
//


/// `interface nsIFind : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFind {
    vtable: *const nsIFindVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFind.
unsafe impl XpCom for nsIFind {
    const IID: nsIID = nsID(0x40aba110, 0x2a56, 0x4678,
        [0xbe, 0x90, 0xe2, 0xc1, 0x7a, 0x9a, 0xe7, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFind {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFind.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFindCoerce {
    /// Cheaply cast a value of this type from a `nsIFind`.
    fn coerce_from(v: &nsIFind) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFindCoerce for nsIFind {
    #[inline]
    fn coerce_from(v: &nsIFind) -> &Self {
        v
    }
}

impl nsIFind {
    /// Cast this `nsIFind` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFindCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFind {
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
impl<T: nsISupportsCoerce> nsIFindCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFind) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFind
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFindVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute boolean findBackwards; */
    pub GetFindBackwards: unsafe extern "system" fn (this: *const nsIFind, aFindBackwards: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean findBackwards; */
    pub SetFindBackwards: unsafe extern "system" fn (this: *const nsIFind, aFindBackwards: bool) -> ::nserror::nsresult,

    /* attribute boolean caseSensitive; */
    pub GetCaseSensitive: unsafe extern "system" fn (this: *const nsIFind, aCaseSensitive: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean caseSensitive; */
    pub SetCaseSensitive: unsafe extern "system" fn (this: *const nsIFind, aCaseSensitive: bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub GetEntireWord: unsafe extern "system" fn (this: *const nsIFind, aEntireWord: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub SetEntireWord: unsafe extern "system" fn (this: *const nsIFind, aEntireWord: bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub GetMatchDiacritics: unsafe extern "system" fn (this: *const nsIFind, aMatchDiacritics: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub SetMatchDiacritics: unsafe extern "system" fn (this: *const nsIFind, aMatchDiacritics: bool) -> ::nserror::nsresult,

    /* Range Find (in AString aPatText, in Range aSearchRange, in Range aStartPoint, in Range aEndPoint); */
    pub Find: unsafe extern "system" fn (this: *const nsIFind, aPatText: *const ::nsstring::nsAString, aSearchRange: *const libc::c_void, aStartPoint: *const libc::c_void, aEndPoint: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFind {


    /// `attribute boolean findBackwards;`
    #[inline]
    pub unsafe fn GetFindBackwards(&self, aFindBackwards: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetFindBackwards)(self, aFindBackwards)
    }



    /// `attribute boolean findBackwards;`
    #[inline]
    pub unsafe fn SetFindBackwards(&self, aFindBackwards: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetFindBackwards)(self, aFindBackwards)
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


    /// ```text
    /// /**
    ///    * Find some text in the current context. The implementation is
    ///    * responsible for performing the find and highlighting the text.
    ///    *
    ///    * @param aPatText     The text to search for.
    ///    * @param aSearchRange A Range specifying domain of search.
    ///    * @param aStartPoint  A Range specifying search start point.
    ///    *                     If not collapsed, we'll start from
    ///    *                     end (forward) or start (backward).
    ///    * @param aEndPoint    A Range specifying search end point.
    ///    *                     If not collapsed, we'll end at
    ///    *                     end (forward) or start (backward).
    ///    * @retval             A range spanning the match that was found (or null).
    ///    */
    /// ```
    ///

    /// `Range Find (in AString aPatText, in Range aSearchRange, in Range aStartPoint, in Range aEndPoint);`
    #[inline]
    pub unsafe fn Find(&self, aPatText: *const ::nsstring::nsAString, aSearchRange: *const libc::c_void, aStartPoint: *const libc::c_void, aEndPoint: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).Find)(self, aPatText, aSearchRange, aStartPoint, aEndPoint, _retval)
    }


}


