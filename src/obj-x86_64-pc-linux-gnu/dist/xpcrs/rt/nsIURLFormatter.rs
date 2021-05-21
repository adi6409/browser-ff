//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/urlformatter/nsIURLFormatter.idl
//


/// `interface nsIURLFormatter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURLFormatter {
    vtable: *const nsIURLFormatterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURLFormatter.
unsafe impl XpCom for nsIURLFormatter {
    const IID: nsIID = nsID(0x4ab31d30, 0x372d, 0x11db,
        [0xa9, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURLFormatter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURLFormatter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURLFormatterCoerce {
    /// Cheaply cast a value of this type from a `nsIURLFormatter`.
    fn coerce_from(v: &nsIURLFormatter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURLFormatterCoerce for nsIURLFormatter {
    #[inline]
    fn coerce_from(v: &nsIURLFormatter) -> &Self {
        v
    }
}

impl nsIURLFormatter {
    /// Cast this `nsIURLFormatter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURLFormatterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURLFormatter {
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
impl<T: nsISupportsCoerce> nsIURLFormatterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURLFormatter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURLFormatter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURLFormatterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString formatURL (in AString aFormat); */
    pub FormatURL: unsafe extern "system" fn (this: *const nsIURLFormatter, aFormat: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString formatURLPref (in AString aPref); */
    pub FormatURLPref: unsafe extern "system" fn (this: *const nsIURLFormatter, aPref: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString trimSensitiveURLs (in AString aMsg); */
    pub TrimSensitiveURLs: unsafe extern "system" fn (this: *const nsIURLFormatter, aMsg: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURLFormatter {

    /// ```text
    /// /**
    ///    * formatURL - Formats a string URL
    ///    *
    ///    * The set of known variables is predefined.
    ///    * If a variable is unknown, it is left unchanged and a non-fatal error is reported.
    ///    *
    ///    * @param aFormat string Unformatted URL.
    ///    *
    ///    * @return The formatted URL.
    ///    */
    /// ```
    ///

    /// `AString formatURL (in AString aFormat);`
    #[inline]
    pub unsafe fn FormatURL(&self, aFormat: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FormatURL)(self, aFormat, _retval)
    }


    /// ```text
    /// /**
    ///    * formatURLPref - Formats a string URL stored in a preference
    ///    *
    ///    * If the preference value cannot be retrieved, a fatal error is reported
    ///    * and the "about:blank" URL is returned.
    ///    *
    ///    * @param aPref string Preference name.
    ///    *
    ///    * @return The formatted URL returned by formatURL(), or "about:blank".
    ///    */
    /// ```
    ///

    /// `AString formatURLPref (in AString aPref);`
    #[inline]
    pub unsafe fn FormatURLPref(&self, aPref: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FormatURLPref)(self, aPref, _retval)
    }


    /// ```text
    /// /**
    ///    * Remove all of the sensitive query parameter strings from URLs in |aMsg|.
    ///    */
    /// ```
    ///

    /// `AString trimSensitiveURLs (in AString aMsg);`
    #[inline]
    pub unsafe fn TrimSensitiveURLs(&self, aMsg: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).TrimSensitiveURLs)(self, aMsg, _retval)
    }


}


