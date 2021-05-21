//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIEnvironment.idl
//


/// `interface nsIEnvironment : nsISupports`
///

/// ```text
/// /**
///  * Scriptable access to the current process environment.
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEnvironment {
    vtable: *const nsIEnvironmentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEnvironment.
unsafe impl XpCom for nsIEnvironment {
    const IID: nsIID = nsID(0x101d5941, 0xd820, 0x4e85,
        [0xa2, 0x66, 0x9a, 0x34, 0x69, 0x94, 0x08, 0x07]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEnvironment {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEnvironment.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEnvironmentCoerce {
    /// Cheaply cast a value of this type from a `nsIEnvironment`.
    fn coerce_from(v: &nsIEnvironment) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEnvironmentCoerce for nsIEnvironment {
    #[inline]
    fn coerce_from(v: &nsIEnvironment) -> &Self {
        v
    }
}

impl nsIEnvironment {
    /// Cast this `nsIEnvironment` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEnvironmentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEnvironment {
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
impl<T: nsISupportsCoerce> nsIEnvironmentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEnvironment) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEnvironment
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEnvironmentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void set (in AString aName, in AString aValue); */
    pub Set: unsafe extern "system" fn (this: *const nsIEnvironment, aName: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString get (in AString aName); */
    pub Get: unsafe extern "system" fn (this: *const nsIEnvironment, aName: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean exists (in AString aName); */
    pub Exists: unsafe extern "system" fn (this: *const nsIEnvironment, aName: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEnvironment {

    /// ```text
    /// /**
    ///      * Set the value of an environment variable.
    ///      *
    ///      * @param aName   the variable name to set.
    ///      * @param aValue  the value to set.
    ///      */
    /// ```
    ///

    /// `void set (in AString aName, in AString aValue);`
    #[inline]
    pub unsafe fn Set(&self, aName: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, aName, aValue)
    }


    /// ```text
    /// /**
    ///      * Get the value of an environment variable.
    ///      *
    ///      * @param aName   the variable name to retrieve.
    ///      * @return        returns the value of the env variable. An empty string
    ///      *                will be returned when the env variable does not exist or
    ///      *                when the value itself is an empty string - please use
    ///      *                |exists()| to probe whether the env variable exists
    ///      *                or not.
    ///      */
    /// ```
    ///

    /// `AString get (in AString aName);`
    #[inline]
    pub unsafe fn Get(&self, aName: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, aName, _retval)
    }


    /// ```text
    /// /**
    ///      * Check the existence of an environment variable.
    ///      * This method checks whether an environment variable is present in
    ///      * the environment or not.
    ///      *
    ///      * - For Unix/Linux platforms we follow the Unix definition:
    ///      * An environment variable exists when |getenv()| returns a non-NULL value.
    ///      * An environment variable does not exist when |getenv()| returns NULL.
    ///      * - For non-Unix/Linux platforms we have to fall back to a
    ///      * "portable" definition (which is incorrect for Unix/Linux!!!!)
    ///      * which simply checks whether the string returned by |Get()| is empty
    ///      * or not.
    ///      *
    ///      * @param aName   the variable name to probe.
    ///      * @return        if the variable has been set, the value returned is
    ///      *                PR_TRUE. If the variable was not defined in the
    ///      *                environment PR_FALSE will be returned.
    ///      */
    /// ```
    ///

    /// `boolean exists (in AString aName);`
    #[inline]
    pub unsafe fn Exists(&self, aName: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Exists)(self, aName, _retval)
    }


}


