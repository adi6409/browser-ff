//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSearch.idl
//


/// `interface nsIAutoCompleteSearch : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteSearch {
    vtable: *const nsIAutoCompleteSearchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteSearch.
unsafe impl XpCom for nsIAutoCompleteSearch {
    const IID: nsIID = nsID(0xde8db85f, 0xc1de, 0x4d87,
        [0x94, 0xba, 0x78, 0x44, 0x89, 0x0f, 0x91, 0xfe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteSearch {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteSearch.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteSearchCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteSearch`.
    fn coerce_from(v: &nsIAutoCompleteSearch) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteSearchCoerce for nsIAutoCompleteSearch {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearch) -> &Self {
        v
    }
}

impl nsIAutoCompleteSearch {
    /// Cast this `nsIAutoCompleteSearch` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSearchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteSearch {
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
impl<T: nsISupportsCoerce> nsIAutoCompleteSearchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearch) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteSearch
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteSearchVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void startSearch (in AString searchString, in AString searchParam, in nsIAutoCompleteResult previousResult, in nsIAutoCompleteObserver listener, [optional] in nsIPropertyBag2 options); */
    pub StartSearch: unsafe extern "system" fn (this: *const nsIAutoCompleteSearch, searchString: *const ::nsstring::nsAString, searchParam: *const ::nsstring::nsAString, previousResult: *const nsIAutoCompleteResult, listener: *const nsIAutoCompleteObserver, options: *const nsIPropertyBag2) -> ::nserror::nsresult,

    /* void stopSearch (); */
    pub StopSearch: unsafe extern "system" fn (this: *const nsIAutoCompleteSearch) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteSearch {


    /// `void startSearch (in AString searchString, in AString searchParam, in nsIAutoCompleteResult previousResult, in nsIAutoCompleteObserver listener, [optional] in nsIPropertyBag2 options);`
    #[inline]
    pub unsafe fn StartSearch(&self, searchString: *const ::nsstring::nsAString, searchParam: *const ::nsstring::nsAString, previousResult: *const nsIAutoCompleteResult, listener: *const nsIAutoCompleteObserver, options: *const nsIPropertyBag2) -> ::nserror::nsresult {
        ((*self.vtable).StartSearch)(self, searchString, searchParam, previousResult, listener, options)
    }



    /// `void stopSearch ();`
    #[inline]
    pub unsafe fn StopSearch(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopSearch)(self, )
    }


}


/// `interface nsIAutoCompleteObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteObserver {
    vtable: *const nsIAutoCompleteObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteObserver.
unsafe impl XpCom for nsIAutoCompleteObserver {
    const IID: nsIID = nsID(0x8bd1dbbc, 0xdcce, 0x4007,
        [0x9a, 0xfa, 0xb5, 0x51, 0xeb, 0x68, 0x7b, 0x61]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteObserver`.
    fn coerce_from(v: &nsIAutoCompleteObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteObserverCoerce for nsIAutoCompleteObserver {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteObserver) -> &Self {
        v
    }
}

impl nsIAutoCompleteObserver {
    /// Cast this `nsIAutoCompleteObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteObserver {
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
impl<T: nsISupportsCoerce> nsIAutoCompleteObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void onSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
    pub OnSearchResult: unsafe extern "system" fn (this: *const nsIAutoCompleteObserver, search: *const nsIAutoCompleteSearch, result: *const nsIAutoCompleteResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteObserver {


    /// `[can_run_script] void onSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result);`
    #[inline]
    pub unsafe fn OnSearchResult(&self, search: *const nsIAutoCompleteSearch, result: *const nsIAutoCompleteResult) -> ::nserror::nsresult {
        ((*self.vtable).OnSearchResult)(self, search, result)
    }


}


/// `interface nsIAutoCompleteSearchDescriptor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteSearchDescriptor {
    vtable: *const nsIAutoCompleteSearchDescriptorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteSearchDescriptor.
unsafe impl XpCom for nsIAutoCompleteSearchDescriptor {
    const IID: nsIID = nsID(0x4c3e7462, 0xfbfb, 0x4310,
        [0x8f, 0x4b, 0x23, 0x92, 0x38, 0x39, 0x2b, 0x75]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteSearchDescriptor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteSearchDescriptor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteSearchDescriptorCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteSearchDescriptor`.
    fn coerce_from(v: &nsIAutoCompleteSearchDescriptor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteSearchDescriptorCoerce for nsIAutoCompleteSearchDescriptor {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearchDescriptor) -> &Self {
        v
    }
}

impl nsIAutoCompleteSearchDescriptor {
    /// Cast this `nsIAutoCompleteSearchDescriptor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSearchDescriptorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteSearchDescriptor {
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
impl<T: nsISupportsCoerce> nsIAutoCompleteSearchDescriptorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearchDescriptor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteSearchDescriptor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteSearchDescriptorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short searchType; */
    pub GetSearchType: unsafe extern "system" fn (this: *const nsIAutoCompleteSearchDescriptor, aSearchType: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute boolean clearingAutoFillSearchesAgain; */
    pub GetClearingAutoFillSearchesAgain: unsafe extern "system" fn (this: *const nsIAutoCompleteSearchDescriptor, aClearingAutoFillSearchesAgain: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteSearchDescriptor {

    pub const SEARCH_TYPE_DELAYED: i64 = 0;


    pub const SEARCH_TYPE_IMMEDIATE: i64 = 1;

    /// ```text
    /// /**
    ///    * Identifies the search behavior.
    ///    * Should be one of the SEARCH_TYPE_* constants above.
    ///    * Defaults to SEARCH_TYPE_DELAYED.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned short searchType;`
    #[inline]
    pub unsafe fn GetSearchType(&self, aSearchType: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchType)(self, aSearchType)
    }



    /// `readonly attribute boolean clearingAutoFillSearchesAgain;`
    #[inline]
    pub unsafe fn GetClearingAutoFillSearchesAgain(&self, aClearingAutoFillSearchesAgain: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetClearingAutoFillSearchesAgain)(self, aClearingAutoFillSearchesAgain)
    }


}


