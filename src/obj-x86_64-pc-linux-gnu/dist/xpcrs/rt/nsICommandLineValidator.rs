//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineValidator.idl
//


/// `interface nsICommandLineValidator : nsISupports`
///

/// ```text
/// /**
///  * Validates arguments on the command line of an XUL application.
///  *
///  * Each validator is registered in the category "command-line-validator".
///  * The entries in this category are read in alphabetical order, and each
///  * category value is treated as a service contractid implementing this
///  * interface.
///  *
///  * By convention, validator with ordinary priority should begin with "m".
///  *
///  * Example:
///  * Category               Entry          Value
///  * command-line-validator b-browser      @mozilla.org/browser/clh;1
///  * command-line-validator m-edit         @mozilla.org/composer/clh;1
///  * command-line-validator m-irc          @mozilla.org/chatzilla/clh;1
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICommandLineValidator {
    vtable: *const nsICommandLineValidatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICommandLineValidator.
unsafe impl XpCom for nsICommandLineValidator {
    const IID: nsIID = nsID(0x5ecaa593, 0x7660, 0x4a3a,
        [0x95, 0x7a, 0x92, 0xd5, 0x77, 0x06, 0x71, 0xc7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICommandLineValidator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICommandLineValidator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICommandLineValidatorCoerce {
    /// Cheaply cast a value of this type from a `nsICommandLineValidator`.
    fn coerce_from(v: &nsICommandLineValidator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICommandLineValidatorCoerce for nsICommandLineValidator {
    #[inline]
    fn coerce_from(v: &nsICommandLineValidator) -> &Self {
        v
    }
}

impl nsICommandLineValidator {
    /// Cast this `nsICommandLineValidator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICommandLineValidatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICommandLineValidator {
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
impl<T: nsISupportsCoerce> nsICommandLineValidatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLineValidator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICommandLineValidator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICommandLineValidatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void validate (in nsICommandLine aCommandLine); */
    pub Validate: unsafe extern "system" fn (this: *const nsICommandLineValidator, aCommandLine: *const nsICommandLine) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICommandLineValidator {

    /// ```text
    /// /**
    ///    * Process the command-line validators in the proper order, calling
    ///    * "validate()" on each.
    ///    *
    ///    * @throws NS_ERROR_ABORT if any validator throws NS_ERROR_ABORT. All other
    ///    *         errors thrown by validators will be silently ignored.
    ///    */
    /// ```
    ///

    /// `void validate (in nsICommandLine aCommandLine);`
    #[inline]
    pub unsafe fn Validate(&self, aCommandLine: *const nsICommandLine) -> ::nserror::nsresult {
        ((*self.vtable).Validate)(self, aCommandLine)
    }


}


