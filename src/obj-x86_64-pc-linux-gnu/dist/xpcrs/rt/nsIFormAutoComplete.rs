//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/satchel/nsIFormAutoComplete.idl
//


/// `interface nsIFormAutoComplete : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFormAutoComplete {
    vtable: *const nsIFormAutoCompleteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFormAutoComplete.
unsafe impl XpCom for nsIFormAutoComplete {
    const IID: nsIID = nsID(0xbfd9b82b, 0x0ab3, 0x4b6b,
        [0x9e, 0x54, 0xaa, 0x96, 0x1f, 0xf4, 0xb7, 0x32]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFormAutoComplete {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFormAutoComplete.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFormAutoCompleteCoerce {
    /// Cheaply cast a value of this type from a `nsIFormAutoComplete`.
    fn coerce_from(v: &nsIFormAutoComplete) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFormAutoCompleteCoerce for nsIFormAutoComplete {
    #[inline]
    fn coerce_from(v: &nsIFormAutoComplete) -> &Self {
        v
    }
}

impl nsIFormAutoComplete {
    /// Cast this `nsIFormAutoComplete` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFormAutoCompleteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFormAutoComplete {
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
impl<T: nsISupportsCoerce> nsIFormAutoCompleteCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormAutoComplete) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFormAutoComplete
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFormAutoCompleteVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void autoCompleteSearchAsync (in AString aInputName, in AString aSearchString, in HTMLInputElement aField, in nsIAutoCompleteResult aPreviousResult, in nsIAutoCompleteResult aDatalistResult, in nsIFormAutoCompleteObserver aListener, [optional] in nsIPropertyBag2 options); */
    pub AutoCompleteSearchAsync: unsafe extern "system" fn (this: *const nsIFormAutoComplete, aInputName: *const ::nsstring::nsAString, aSearchString: *const ::nsstring::nsAString, aField: *const libc::c_void, aPreviousResult: *const nsIAutoCompleteResult, aDatalistResult: *const nsIAutoCompleteResult, aListener: *const nsIFormAutoCompleteObserver, options: *const nsIPropertyBag2) -> ::nserror::nsresult,

    /* void stopAutoCompleteSearch (); */
    pub StopAutoCompleteSearch: unsafe extern "system" fn (this: *const nsIFormAutoComplete) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFormAutoComplete {

    /// ```text
    /// /**
    ///      * Generate results for a form input autocomplete menu asynchronously.
    ///      */
    /// ```
    ///

    /// `void autoCompleteSearchAsync (in AString aInputName, in AString aSearchString, in HTMLInputElement aField, in nsIAutoCompleteResult aPreviousResult, in nsIAutoCompleteResult aDatalistResult, in nsIFormAutoCompleteObserver aListener, [optional] in nsIPropertyBag2 options);`
    #[inline]
    pub unsafe fn AutoCompleteSearchAsync(&self, aInputName: *const ::nsstring::nsAString, aSearchString: *const ::nsstring::nsAString, aField: *const libc::c_void, aPreviousResult: *const nsIAutoCompleteResult, aDatalistResult: *const nsIAutoCompleteResult, aListener: *const nsIFormAutoCompleteObserver, options: *const nsIPropertyBag2) -> ::nserror::nsresult {
        ((*self.vtable).AutoCompleteSearchAsync)(self, aInputName, aSearchString, aField, aPreviousResult, aDatalistResult, aListener, options)
    }


    /// ```text
    /// /**
    ///      * If a search is in progress, stop it. Otherwise, do nothing. This is used
    ///      * to cancel an existing search, for example, in preparation for a new search.
    ///      */
    /// ```
    ///

    /// `void stopAutoCompleteSearch ();`
    #[inline]
    pub unsafe fn StopAutoCompleteSearch(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopAutoCompleteSearch)(self, )
    }


}


/// `interface nsIFormAutoCompleteObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFormAutoCompleteObserver {
    vtable: *const nsIFormAutoCompleteObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFormAutoCompleteObserver.
unsafe impl XpCom for nsIFormAutoCompleteObserver {
    const IID: nsIID = nsID(0x604419ab, 0x55a0, 0x4831,
        [0x9e, 0xca, 0x1b, 0x9e, 0x67, 0xcc, 0x47, 0x51]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFormAutoCompleteObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFormAutoCompleteObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFormAutoCompleteObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIFormAutoCompleteObserver`.
    fn coerce_from(v: &nsIFormAutoCompleteObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFormAutoCompleteObserverCoerce for nsIFormAutoCompleteObserver {
    #[inline]
    fn coerce_from(v: &nsIFormAutoCompleteObserver) -> &Self {
        v
    }
}

impl nsIFormAutoCompleteObserver {
    /// Cast this `nsIFormAutoCompleteObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFormAutoCompleteObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFormAutoCompleteObserver {
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
impl<T: nsISupportsCoerce> nsIFormAutoCompleteObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormAutoCompleteObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFormAutoCompleteObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFormAutoCompleteObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void onSearchCompletion (in nsIAutoCompleteResult result); */
    pub OnSearchCompletion: unsafe extern "system" fn (this: *const nsIFormAutoCompleteObserver, result: *const nsIAutoCompleteResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFormAutoCompleteObserver {


    /// `[can_run_script] void onSearchCompletion (in nsIAutoCompleteResult result);`
    #[inline]
    pub unsafe fn OnSearchCompletion(&self, result: *const nsIAutoCompleteResult) -> ::nserror::nsresult {
        ((*self.vtable).OnSearchCompletion)(self, result)
    }


}


