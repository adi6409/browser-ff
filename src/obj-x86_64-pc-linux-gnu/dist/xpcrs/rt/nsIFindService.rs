//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIFindService.idl
//


/// `interface nsIFindService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFindService {
    vtable: *const nsIFindServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFindService.
unsafe impl XpCom for nsIFindService {
    const IID: nsIID = nsID(0x5060b801, 0x340e, 0x11d5,
        [0xbe, 0x5b, 0xb3, 0xe0, 0x63, 0xec, 0x6a, 0x3c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFindService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFindService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFindServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIFindService`.
    fn coerce_from(v: &nsIFindService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFindServiceCoerce for nsIFindService {
    #[inline]
    fn coerce_from(v: &nsIFindService) -> &Self {
        v
    }
}

impl nsIFindService {
    /// Cast this `nsIFindService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFindServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFindService {
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
impl<T: nsISupportsCoerce> nsIFindServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFindService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFindService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFindServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AString searchString; */
    pub GetSearchString: unsafe extern "system" fn (this: *const nsIFindService, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString searchString; */
    pub SetSearchString: unsafe extern "system" fn (this: *const nsIFindService, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString replaceString; */
    pub GetReplaceString: unsafe extern "system" fn (this: *const nsIFindService, aReplaceString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString replaceString; */
    pub SetReplaceString: unsafe extern "system" fn (this: *const nsIFindService, aReplaceString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean findBackwards; */
    pub GetFindBackwards: unsafe extern "system" fn (this: *const nsIFindService, aFindBackwards: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean findBackwards; */
    pub SetFindBackwards: unsafe extern "system" fn (this: *const nsIFindService, aFindBackwards: bool) -> ::nserror::nsresult,

    /* attribute boolean wrapFind; */
    pub GetWrapFind: unsafe extern "system" fn (this: *const nsIFindService, aWrapFind: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean wrapFind; */
    pub SetWrapFind: unsafe extern "system" fn (this: *const nsIFindService, aWrapFind: bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub GetEntireWord: unsafe extern "system" fn (this: *const nsIFindService, aEntireWord: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean entireWord; */
    pub SetEntireWord: unsafe extern "system" fn (this: *const nsIFindService, aEntireWord: bool) -> ::nserror::nsresult,

    /* attribute boolean matchCase; */
    pub GetMatchCase: unsafe extern "system" fn (this: *const nsIFindService, aMatchCase: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean matchCase; */
    pub SetMatchCase: unsafe extern "system" fn (this: *const nsIFindService, aMatchCase: bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub GetMatchDiacritics: unsafe extern "system" fn (this: *const nsIFindService, aMatchDiacritics: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean matchDiacritics; */
    pub SetMatchDiacritics: unsafe extern "system" fn (this: *const nsIFindService, aMatchDiacritics: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFindService {


    /// `attribute AString searchString;`
    #[inline]
    pub unsafe fn GetSearchString(&self, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchString)(self, aSearchString)
    }



    /// `attribute AString searchString;`
    #[inline]
    pub unsafe fn SetSearchString(&self, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchString)(self, aSearchString)
    }



    /// `attribute AString replaceString;`
    #[inline]
    pub unsafe fn GetReplaceString(&self, aReplaceString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetReplaceString)(self, aReplaceString)
    }



    /// `attribute AString replaceString;`
    #[inline]
    pub unsafe fn SetReplaceString(&self, aReplaceString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetReplaceString)(self, aReplaceString)
    }



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



    /// `attribute boolean wrapFind;`
    #[inline]
    pub unsafe fn GetWrapFind(&self, aWrapFind: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWrapFind)(self, aWrapFind)
    }



    /// `attribute boolean wrapFind;`
    #[inline]
    pub unsafe fn SetWrapFind(&self, aWrapFind: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetWrapFind)(self, aWrapFind)
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



    /// `attribute boolean matchCase;`
    #[inline]
    pub unsafe fn GetMatchCase(&self, aMatchCase: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchCase)(self, aMatchCase)
    }



    /// `attribute boolean matchCase;`
    #[inline]
    pub unsafe fn SetMatchCase(&self, aMatchCase: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetMatchCase)(self, aMatchCase)
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


}


